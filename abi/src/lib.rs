mod config;
mod error;
mod pb;
mod utils;

pub use config::{Config, DbConfig};
pub use error::Error;
pub use pb::*;
use sqlx::{postgres::PgRow, FromRow, Row};
pub use utils::*;

impl FromRow<'_, PgRow> for pb::Vocabulary {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");
        let updated_at: chrono::DateTime<chrono::Utc> = row.get("updated_at");

        Ok(Self {
            id: row.get("id"),
            word: row.get("word"),
            soundmark: row.get("soundmark"),
            roots: row.get("roots"),
            paraphrase: row.get("paraphrase"),
            collocations: row.get("collocations"),
            synonyms: row.get("synonyms"),
            examples: row.get("examples"),
            created_at: Some(convert_to_timestamp(&created_at)),
            updated_at: Some(convert_to_timestamp(&updated_at)),
        })
    }
}
