use base64::Engine;
use postgres::{Client, Config, NoTls};
use std::env;
use std::error::Error;
use std::time::Duration;
use crypt::{cryptography, BASE64};

refinery::embed_migrations!("migrations");

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let mut postgres_config = Config::new();
    postgres_config.user("user");
    postgres_config.password(get_shop_password()?);
    postgres_config.dbname("shop");
    postgres_config.host("localhost");
    postgres_config.port(5432);
    postgres_config.connect_timeout(Duration::from_secs(5));

    let mut client: Client = postgres_config.connect(NoTls)?;
    check_connection(&mut client)?;

    // The "migrations" module is created by the "embed_migrations" macro
    let report = migrations::runner().run(&mut client).unwrap();
    for migration in report.applied_migrations() {
        log::info!("Migration applied: {}", migration)
    }

    Ok(())
}

fn check_connection(client: &mut Client) -> Result<(), postgres::Error> {
    let connection_timeout = Duration::from_secs(5);
    if let Err(e) = client.is_valid(connection_timeout) {
        log::error!(
			"Failed to establish a connection to Postgres within {} seconds; {}",
			connection_timeout.as_secs(),
			e
		);
        return Err(e);
    }
    Ok(())
}

fn get_shop_password() -> Result<String, Box<dyn Error>> {
    let master_secret: Vec<u8> = get_master_secret()?;
    let secret: Vec<u8> = cryptography::decrypt(&master_secret, "postgres__user.shop.password")?;
    let secret: String = String::from_utf8(secret)?;
    Ok(secret)
}

/// Decrypt a secret managed by the `crypt` application.
fn get_master_secret() -> Result<Vec<u8>, Box<dyn Error>> {
    let variable_name = "MASTER_SECRET";
    let master_secret: String = env::var(variable_name)?;
    let master_secret: Vec<u8> = BASE64.decode(master_secret)?;
    Ok(master_secret)
}
