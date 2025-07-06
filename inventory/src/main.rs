use inventory::{ebay, environment, server};
use sqlx::{Pool, Postgres};
use std::io;

#[actix_web::main]
async fn main() -> Result<(), io::Error> {
    environment::load_env()?;
    environment::init_logger()
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    log::info!("Runtime environment: {:?}", environment::RuntimeEnvironment::default());

    let pgpool: Pool<Postgres> = inventory::db::sqlx_connect().await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    ebay::ebay_action::init(&pgpool).await;

    server::open_server(pgpool).await
}
