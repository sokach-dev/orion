mod vocabulary;

use async_trait::async_trait;
use sqlx::PgPool;

#[derive(Debug)]
pub struct OrionService {
    pool: PgPool,
}

#[async_trait]
pub trait VocabularyTrait {
    /// make a vocabulary
    async fn add_vocabulary(&self, v: abi::Vocabulary) -> Result<abi::Vocabulary, abi::Error>;
}
