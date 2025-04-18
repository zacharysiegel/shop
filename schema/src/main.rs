use std::time::Duration;

use postgres::{Client, Config, NoTls};

refinery::embed_migrations!("migrations");

fn main() -> Result<(), postgres::Error> {
	env_logger::builder()
		.filter_level(log::LevelFilter::Info)
		.init();

	let mut postgres_config = Config::new();
	postgres_config.user("user");
	postgres_config.password("password");
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

	Result::Ok(())
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
