use crate::ModelService;
use sqlx::Row;
use tonic::async_trait;

#[async_trait]
pub trait StoryTrait {
    /// make a story
    async fn add_story(&self, mut s: abi::Story) -> Result<abi::Story, abi::Error>;
}

#[async_trait]
impl StoryTrait for ModelService {
    /// make a story
    async fn add_story(&self, mut s: abi::Story) -> Result<abi::Story, abi::Error> {
        let id: i64 =
            sqlx::query(r#"INSERT INTO story (words, content) VALUES ($1, $2) RETURNING id"#)
                .bind(s.words.clone())
                .bind(s.content.clone())
                .fetch_one(&self.pool)
                .await?
                .get(0);

        s.id = id;
        Ok(s)
    }
}
