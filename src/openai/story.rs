use abi::{story_service_client::StoryServiceClient, QueryVocabularyRandomRequest};
use anyhow::anyhow;
use crossbeam_channel::{bounded, Receiver};
use crossbeam_utils::sync::WaitGroup;
use defer_lite::defer;
use tonic::transport::Channel;

use super::OpenAI;

async fn generate_story_with_word_list(
    r: Receiver<Option<()>>,
    _wg: WaitGroup,
    ws: Vec<String>,
    openai_url: String,
    key: String,
    prompt: String,
    mut rpc_client: StoryServiceClient<Channel>,
) -> Result<abi::Story, abi::Error> {
    defer! {
        _ = r.recv();
    }
    let request = openai::OpenAIRequestBuilder::default()
        .model("gpt-3.5-turbo".to_string())
        .temperature(1.0)
        .messages(vec![
            openai::MessageBuilder::default()
                .role("system")
                .content(prompt.clone())
                .build()
                .unwrap(),
            openai::MessageBuilder::default()
                .role("user")
                .content(ws.clone().join(","))
                .build()
                .unwrap(),
        ])
        .build()
        .unwrap();

    let response = openai::get_from_openai(&openai_url, &key, &request).await?;

    let v = response.parse_content::<abi::AddStoryRequest, _>(|content| {
        let content = content.trim();

        Ok(abi::AddStoryRequest {
            story: Some(abi::Story {
                id: 0,
                words: ws.clone(),
                content: content.to_string(),
                read_count: 0,
                created_at: None,
                updated_at: None,
            }),
        })
    })?;

    let response = rpc_client.add_story(v).await?.into_inner();

    if response.story.is_none() {
        return Err(anyhow!("add story: {:?} failed", ws).into());
    }
    Ok(response.story.unwrap())
}

impl OpenAI {
    /// select words, that will generate story from openai
    pub async fn generate_story_with_words(
        &self,
        word_amount: i64,
        total: u32,
    ) -> Result<(), abi::Error> {
        let (s, r) = bounded(self.concurrence_amount);
        let wg = WaitGroup::new();
        // 1. get some word from db
        for _ in 0..total {
            let request = tonic::Request::new(QueryVocabularyRandomRequest { limit: word_amount });

            let response = self
                .get_vocabulary_client()
                .await?
                .query_vocabulary_random(request)
                .await?
                .into_inner();

            let mut word_list = vec![];
            for v in response.vocabulary {
                word_list.push(v.word);
            }

            if s.send(None).is_ok() {
                let wg = wg.clone();
                let rr = r.clone();
                let openai_url = self.openai_url.clone();
                let key = self.key.clone();
                let prompt = self.prompt.clone();
                let rpc = self.get_story_client().await?;

                tokio::spawn(async move {
                    tracing::info!("will add story, with words : {:?}", word_list);
                    match generate_story_with_word_list(
                        rr,
                        wg,
                        word_list.clone(),
                        openai_url,
                        key,
                        prompt,
                        rpc,
                    )
                    .await
                    {
                        Ok(s) => tracing::info!("success add story: {:?}", s),
                        Err(e) => tracing::error!("failed add story: {}", e),
                    }
                });
            }
        }
        Ok(())
    }
}
