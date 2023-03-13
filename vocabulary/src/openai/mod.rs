use anyhow::anyhow;
use crossbeam_channel::{bounded, Receiver};
use crossbeam_utils::sync::WaitGroup;
use regex::Regex;
use service::{OrionService, VocabularyTrait};
use std::fs;

async fn add_word_to_db(
    r: Receiver<Option<()>>,
    _wg: WaitGroup,
    w: String,
    db: OrionService,
    url: String,
    key: String,
    prompt: String,
) -> Result<abi::Vocabulary, abi::Error> {
    let request = openai::OpenAIRequestBuilder::default()
        .model("gpt-3.5-turbo".to_string())
        .temperature(0.2)
        .messages(vec![
            openai::MessageBuilder::default()
                .role("system")
                .content(prompt.to_string())
                .build()
                .unwrap(),
            openai::MessageBuilder::default()
                .role("user")
                .content(w)
                .build()
                .unwrap(),
        ])
        .build()
        .unwrap();

    let response = openai::get_from_openai(&url, &key, &request).await?;

    let v = response.parse_content::<abi::Vocabulary, _>(|content| {
        let re = Regex::new(
            r#"单词：(?P<word>.+)(\n)+英式音标：(?P<soundmark>.+)(\n)+词根词缀：(?P<roots>.+)(\n)+中文释义：(?P<paraphrase>.+)(\n)+常用搭配：(?P<collocations>.+)(\n)+近义词：(?P<synonyms>.+)(\n)+例句：(?P<examples>(.|\s)+)"#,
        )?;
        let caps = re.captures(&content).ok_or(anyhow!("openai no match content: {}", &content))?;

        Ok(abi::Vocabulary {
            id: 0,
            word: caps.name("word").ok_or(anyhow!("can't match word"))?.as_str().into(),
            soundmark: caps.name("soundmark").ok_or(anyhow!("can't match soundmark"))?.as_str().into(),
            roots: caps.name("roots").ok_or(anyhow!("can't match roots"))?.as_str().into(),
            paraphrase: caps.name("paraphrase").ok_or(anyhow!("can't match paraphrase"))?.as_str().into(),
            collocations: caps.name("collocations").ok_or(anyhow!("can't match collocations"))?.as_str().into(),
            synonyms: caps.name("synonyms").ok_or(anyhow!("can't match synonyms"))?.as_str().into(),
            examples: caps.name("examples").ok_or(anyhow!("can't match examples"))?.as_str().into(),
            created_at: None,
            updated_at: None,
        })
    })?;

    let v = db.add_vocabulary(v).await?;
    _ = r.recv();
    Ok(v)
}

/// Add words to the database from word list file.
pub async fn add_word_from_file(
    db_config: abi::DbConfig,
    key: &str,
    url: &str,
    word_list_file: &str,
    prompt: &str,
) -> Result<(), abi::Error> {
    let db = service::OrionService::from_config(&db_config).await?;

    // get word list from file and split it by new line
    let word_list = fs::read_to_string(word_list_file)?;
    let word_list = word_list
        .split('\n')
        .map(|s| s.into())
        .collect::<Vec<String>>();

    let (s, r) = bounded(20);

    let wg = WaitGroup::new();

    for w in word_list {
        if s.send(None).is_ok() {
            let wg = wg.clone();
            let rr = r.clone();
            let url = url.to_string();
            let key = key.to_string();
            let prompt = prompt.to_string();
            let db = db.clone();
            tokio::spawn(async move {
                tracing::info!("{}", w);
                let v = add_word_to_db(rr, wg, w, db, url, key, prompt).await;
                tracing::debug!("add new word: {:?}", v);
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn match_should_work() {
        let content = "单词：apple\n英式音标：/ˈæpl/\n词根词缀：无\n中文释义：苹果\n常用搭配：an apple a day (每天一苹果)\n近义词：无\n例句：I like to eat apples for breakfast. (我喜欢在早餐时吃苹果。)";
        let re = Regex::new(
            r#"单词：(?P<word>.+)(\n)+英式音标：(?P<phonetic>.+)(\n)+词根词缀：(?P<root>.+)(\n){1,2}中文释义：(?P<chinese>.+)(\n){1,2}常用搭配：(?P<collocation>.+)(\n){1,2}近义词：(?P<synonym>.+)(\n){1,2}例句：(?P<example>(.|\s)+)"#,
        )
        .unwrap();
        let caps = re.captures(&content).unwrap();

        assert_eq!(caps.name("word").unwrap().as_str(), "apple");
        assert_eq!(caps.name("root").unwrap().as_str(), "无");
    }
}
