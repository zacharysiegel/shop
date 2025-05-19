use inventory::environment::RuntimeEnvironment;
use log::LevelFilter;

#[actix_web::main]
async fn main() -> Result<(), impl std::error::Error> {
    inventory::environment::load_env()?;

    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .filter_module("actix_server", LevelFilter::Debug)
        .filter_module("actix_web::middleware::logger", LevelFilter::Warn)
        .init();

    log::info!("Runtime environment: {:?}", RuntimeEnvironment::default());

    storefront::server::open_server().await
}
