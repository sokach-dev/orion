mod rpc;
pub mod vocabulary;

use abi::vocabulary_service_server::VocabularyServiceServer;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct ModelService {
    pool: PgPool,
}

impl ModelService {
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

#[derive(Debug, Clone)]
pub struct OrionService {
    model: ModelService,
}

impl OrionService {
    pub async fn from_config(config: &abi::DbConfig) -> Result<Self, sqlx::Error> {
        Ok(Self {
            model: ModelService::from_config(config).await?,
        })
    }
}

pub async fn start_server(config: &abi::Config) -> Result<(), Box<dyn std::error::Error>> {
    let addr: std::net::SocketAddr = format!("{}:{}", config.host, config.port).parse()?;

    let svc = OrionService::from_config(&config.db_config).await?;
    let svc = VocabularyServiceServer::new(svc);

    tracing::info!("Listening on {}", addr);

    tonic::transport::Server::builder()
        .add_service(svc)
        .serve(addr)
        .await?;
    Ok(())
}
