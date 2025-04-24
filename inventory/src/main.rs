use log::LevelFilter;
use sqlx::{Pool, Postgres};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    inventory::env::load_env()?;

    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .filter_module("actix_server", LevelFilter::Debug)
        .filter_module("actix_web::middleware::logger", LevelFilter::Warn)
        .init();

    let pgpool: Pool<Postgres> = inventory::db::sqlx_connect().await?;
    inventory::server::open_server(pgpool).await
}
