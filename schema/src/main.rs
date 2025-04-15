use std::net::{IpAddr, Ipv4Addr};

use log::info;
use refinery::Migration;

refinery::embed_migrations!("migrations");

fn main() -> Result<(), postgres::Error> {
    env_logger::init();

    let mut config = postgres::Config::new();
    config.user("user");
    config.password("password");
    config.dbname("connect");
    config.hostaddr(IpAddr::V4(Ipv4Addr::new(172, 0, 0, 1)));
    config.port(5432);

    let mut client: postgres::Client = config.connect(postgres::NoTls)?;
    migrations::runner().run(&mut client).unwrap();

    return Result::Ok(());
}

fn process_migration(migration: Migration) {
    #[cfg(not(feature = "enums"))]
    {
        // run something after each migration
        info!("Post-processing a migration: {}", migration)
    }

    #[cfg(feature = "enums")]
    {
        // or with the `enums` feature enabled, match against migrations to run specific post-migration steps
        use migrations::EmbeddedMigration;
        match migration.into() {
            EmbeddedMigration::Initial(m) => info!("V{}: Initialized the database!", m.version()),
            m => info!("Got a migration: {:?}", m),
        }
    }
}
