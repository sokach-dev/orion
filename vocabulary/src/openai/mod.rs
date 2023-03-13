use anyhow::anyhow;
use regex::Regex;
use service::VocabularyTrait;
use std::fs;

/// Add words to the database from word list file.
pub async fn add_word_to_db(
    db_config: abi::DbConfig,
    key: &str,
    url: &str,
    word_list_file: &str,
    prompt: &str,
) -> Result<(), abi::Error> {
    let db = service::OrionService::from_config(&db_config).await?;

    // get word list from file and split it by new line
    let word_list = fs::read_to_string(word_list_file)?;
    let word_list = word_list.split('\n').collect::<Vec<&str>>();

    for w in word_list {
        tracing::info!("{}", w);

        let request = openai::OpenAIRequestBuilder::default()
            .model("gpt-3.5-turbo".to_string())
            .temperature(0.2)
            .message(vec![
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

        let response = openai::get_from_openai(url, key, &request).await?;

        let v = response.parse_content::<abi::Vocabulary, _>(|content| {
            let re = Regex::new(
                r#"单词：(?P<word>.+)\n\n英式音标：(?P<phonetic>.+)\n\n词根词缀：(?P<root>.+)\n\n中文释义：(?P<chinese>.+)\n\n常用搭配：(?P<collocation>.+)\n\n近义词：(?P<synonym>.+)\n\n例句：(?P<example>(.|\s)+)"#,
            )?;
            let caps = re.captures(&content).ok_or(anyhow!("openai no match content"))?;

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

        db.add_vocabulary(v).await?;
    }
    Ok(())
}
