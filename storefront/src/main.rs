use std::io;
use log::LevelFilter;

#[actix_web::main]
async fn main() -> Result<(), impl std::error::Error> {
    load_env()?;

    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .filter_module("actix_server", LevelFilter::Debug)
        .filter_module("actix_web::middleware::logger", LevelFilter::Warn)
        .init();

    storefront::server::open_server().await
}

fn load_env() -> Result<(), io::Error> {
    match dotenvy::dotenv() {
        Ok(_) => Ok(()),
        Err(error) => Err(io::Error::new(io::ErrorKind::Other, error))?,
    }
}
