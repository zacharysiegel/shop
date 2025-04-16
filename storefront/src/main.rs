#[actix_web::main]
async fn main() -> Result<(), impl std::error::Error> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    storefront::server::open_server().await
}