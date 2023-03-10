mod vocabulary;

use async_trait::async_trait;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct OrionService {
    pool: PgPool,
}

#[async_trait]
pub trait VocabularyTrait {
    /// make a vocabulary
    async fn add_vocabulary(&self, v: abi::Vocabulary) -> Result<abi::Vocabulary, abi::Error>;

    /// get a vocabulary
    async fn get_vocabulary(
        &self,
        q: abi::VocabularyQuery,
    ) -> Result<Vec<abi::Vocabulary>, abi::Error>;
}
