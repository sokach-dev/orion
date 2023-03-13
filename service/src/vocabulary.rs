use crate::{OrionService, VocabularyTrait};
use async_trait::async_trait;

impl OrionService {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
    pub async fn from_config(config: &abi::DbConfig) -> Result<Self, sqlx::Error> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database_url)
            .await?;
        Ok(Self::new(pool))
    }
}

#[async_trait]
impl VocabularyTrait for OrionService {
    async fn add_vocabulary(&self, mut v: abi::Vocabulary) -> Result<abi::Vocabulary, abi::Error> {
        let id = sqlx::query!(
            r#"
            INSERT INTO vocabulary (word, soundmark, roots, paraphrase, collocations, synonyms, examples)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
            "#,
            v.word,
            v.soundmark,
            v.roots,
            v.paraphrase,
            v.collocations,
            v.synonyms,
            v.examples
        )
        .fetch_one(&self.pool)
        .await?
        .id;

        v.id = id;
        Ok(v)
    }
}
