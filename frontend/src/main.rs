use inventory::environment::RuntimeEnvironment;

#[actix_web::main]
async fn main() -> Result<(), impl std::error::Error> {
    inventory::environment::load_env()?;
    inventory::environment::init_logger()
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;

    log::info!("Runtime environment: {:?}", RuntimeEnvironment::default());

    frontend::server::open_server().await
}
