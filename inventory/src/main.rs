use inventory::{environment, marketplace, server};
use log::LevelFilter;
use sqlx::{Pool, Postgres};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    environment::load_env()?;

    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .filter_module("actix_server", LevelFilter::Debug)
        .filter_module("actix_web::middleware::logger", LevelFilter::Info)
        .init();

    log::info!("Runtime environment: {:?}", environment::RuntimeEnvironment::default());

    let pgpool: Pool<Postgres> = inventory::db::sqlx_connect().await?;

    marketplace::ebay::ebay_action::init(&pgpool).await;

    server::open_server(pgpool).await
}
