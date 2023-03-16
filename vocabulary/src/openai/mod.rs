use abi::vocabulary_service_client::VocabularyServiceClient;
use abi::{QueryVocabularyRequest, VocabularyQuery};
use anyhow::anyhow;
use crossbeam_channel::{bounded, Receiver};
use crossbeam_utils::sync::WaitGroup;
use defer_lite::defer;
use regex::Regex;
use std::fs;
use tonic::transport::Channel;

pub struct OpenAI {
    pub rpc: String,
    pub key: String,
    pub openai_url: String,
    pub word_list_file: String,
    pub prompt: String,
}

async fn add_word_to_db(
    r: Receiver<Option<()>>,
    _wg: WaitGroup,
    w: String,
    openai_url: String,
    key: String,
    prompt: String,
    mut rpc_client: VocabularyServiceClient<Channel>,
) -> Result<abi::Vocabulary, abi::Error> {
    defer! {
        _ = r.recv();
    }
    let request = openai::OpenAIRequestBuilder::default()
        .model("gpt-3.5-turbo".to_string())
        .temperature(0.2)
        .messages(vec![
            openai::MessageBuilder::default()
                .role("system")
                .content(prompt.clone())
                .build()
                .unwrap(),
            openai::MessageBuilder::default()
                .role("user")
                .content(w.clone())
                .build()
                .unwrap(),
        ])
        .build()
        .unwrap();

    let response = openai::get_from_openai(&openai_url, &key, &request).await?;

    let v = response.parse_content::<abi::AddVocabularyRequest, _>(|content| {
            let re = Regex::new(
                r#"单词：(?P<word>.+)(\n)+英式音标：(?P<soundmark>.+)(\n)+词根词缀：(?P<roots>.+)(\n)+中文释义：(?P<paraphrase>.+)(\n)+常用搭配：(?P<collocations>.+)(\n)+近义词：(?P<synonyms>.+)(\n)+例句：(?P<examples>(.|\s)+)"#,
            )?;
            let content = content.trim();
            let caps = re.captures(&content).ok_or(anyhow!("openai no match content: {}", &content))?;

            Ok(abi::AddVocabularyRequest{
                vocabulary: Some( abi::Vocabulary{
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
                }
                )
            })
        })?;

    let response = rpc_client.add_vocabulary(v).await?.into_inner();

    if response.vocabulary.is_none() {
        return Err(anyhow!("add word: {} failed", w).into());
    }
    Ok(response.vocabulary.unwrap())
}

impl OpenAI {
    pub async fn new(
        rpc_server: String,
        key: String,
        openai_url: String,
        word_list_file: String,
        prompt: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            rpc: rpc_server,
            key,
            openai_url,
            word_list_file,
            prompt,
        })
    }

    async fn get_client(&self) -> Result<VocabularyServiceClient<Channel>, abi::Error> {
        let rpc = VocabularyServiceClient::connect(self.rpc.clone()).await?;
        Ok(rpc)
    }

    /// Add words to the database from word list file.
    pub async fn add_word_from_file(&self) -> Result<(), abi::Error> {
        // get word list from file and split it by new line
        let word_list = fs::read_to_string(self.word_list_file.as_str())?;
        let word_list = word_list
            .split('\n')
            .map(|s| s.into())
            .collect::<Vec<String>>();

        let concurrence_num_str =
            std::env::var("OPENAI_CONCURRENCE_NUM").unwrap_or("2".to_string());
        let num = concurrence_num_str.parse::<usize>().unwrap_or(2);
        let (s, r) = bounded(num); // openai request limit 20/min

        let wg = WaitGroup::new();

        for w in word_list {
            // check this word is exist in db
            let request = tonic::Request::new(QueryVocabularyRequest {
                query: Some(VocabularyQuery {
                    word: Some(w.clone()),
                    ..Default::default()
                }),
            });

            let response = self
                .get_client()
                .await?
                .query_vocabulary(request)
                .await?
                .into_inner();

            if response.vocabulary.len() > 0 {
                tracing::warn!("Word: {} is exist in db", w);
                continue;
            }

            if s.send(None).is_ok() {
                let wg = wg.clone();
                let rr = r.clone();
                let openai_url = self.openai_url.clone();
                let key = self.key.clone();
                let prompt = self.prompt.clone();
                let rpc = self.get_client().await?;
                tokio::spawn(async move {
                    tracing::info!("will add word: {}", w);
                    match add_word_to_db(rr, wg, w.clone(), openai_url, key, prompt, rpc).await {
                        Ok(v) => tracing::info!("success add word: {:?}", v),
                        Err(e) => tracing::error!("failed add word: {}, reason: {:?}", w, e),
                    }
                });
            }
        }
        Ok(())
    }
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
