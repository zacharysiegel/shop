use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, Pool, Postgres};

mod category;

pub async fn sqlx_connect() -> Result<Pool<Postgres>, std::io::Error> {
	let pool_result: Result<Pool<Postgres>, Error> = PgPoolOptions::new()
		.max_connections(16)
		.connect("postgres://user:password@localhost:5432/shop")
		.await;

	match (pool_result) {
		Ok(pool) => Ok(pool),
		Err(error) => Err(std::io::Error::new(std::io::ErrorKind::Other, error)),
	}
}
