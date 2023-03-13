#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("unknown error")]
    Unknown,

    #[error("anyhow error: {0}")]
    AnyhowError(#[from] anyhow::Error),
}
