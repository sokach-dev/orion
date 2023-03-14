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
    // add new vocabulary
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

    // get vocabularys
    async fn get_vocabulary(
        &self,
        q: abi::VocabularyQuery,
    ) -> Result<Vec<abi::Vocabulary>, abi::Error> {
        let condition = q.to_sql_condition();
        tracing::debug!("condition: {}", condition);
        let sql = format!("SELECT * FROM vocabulary {}", condition);
        println!("condition: {}", sql);

        let result: Vec<abi::Vocabulary> = sqlx::query_as(&sql).fetch_all(&self.pool).await?;

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    async fn get_db(
        pool: &sqlx::PgPool,
        word: String,
        soundmark: String,
        roots: String,
        paraphrase: String,
        collocations: String,
        synonyms: String,
        examples: String,
    ) -> Result<(OrionService, abi::Vocabulary), abi::Error> {
        let db = OrionService::new(pool.clone());
        let v = abi::Vocabulary {
            id: 0,
            word,
            soundmark,
            roots,
            paraphrase,
            collocations,
            synonyms,
            examples,
            created_at: None,
            updated_at: None,
        };
        let v = db.add_vocabulary(v).await?;
        Ok((db, v))
    }

    async fn get_apple_db(
        pool: &sqlx::PgPool,
    ) -> Result<(OrionService, abi::Vocabulary), abi::Error> {
        get_db(
            pool,
            "apple".into(),
            "apple".into(),
            "无".into(),
            "苹果".into(),
            "an apple".into(),
            "无".into(),
            "i have an apple".into(),
        )
        .await
    }

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn get_vocabulary_should_work() {
        let (db, _v) = get_apple_db(&migrated_pool).await.unwrap();
        let q = abi::VocabularyQueryBuilder::default()
            .word(Some("apple".into()))
            .build()
            .unwrap();

        let records = db.get_vocabulary(q).await.unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].word, "apple".to_string());
    }
}
