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

    #[error("tonic trasnport error: {0}")]
    TonicTransportError(#[from] tonic::transport::Error),

    #[error("tonic status: {0}")]
    TonicStatus(#[from] tonic::Status),
}

impl From<Error> for tonic::Status {
    fn from(e: Error) -> Self {
        match e {
            Error::SqlxError(_) => tonic::Status::internal(e.to_string()),
            Error::Unknown => tonic::Status::unknown("unknown error"),
            Error::IOError(_) | Error::AnyhowError(_) | Error::TonicTransportError(_) => {
                tonic::Status::internal(e.to_string())
            }
            Error::TonicStatus(s) => s,
        }
    }
}
