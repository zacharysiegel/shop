use inventory::{environment, marketplace, server};
use sqlx::{Pool, Postgres};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    environment::load_env()?;
    environment::init_logger()
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;

    log::info!("Runtime environment: {:?}", environment::RuntimeEnvironment::default());

    let pgpool: Pool<Postgres> = inventory::db::sqlx_connect().await?;

    marketplace::ebay::ebay_action::init(&pgpool).await;

    server::open_server(pgpool).await
}
