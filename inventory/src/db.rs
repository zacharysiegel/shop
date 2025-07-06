use crate::decrypt::master_decrypt;
use crate::error::ShopError;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, Pool, Postgres};

pub async fn sqlx_connect() -> Result<Pool<Postgres>, ShopError> {
    let password: Vec<u8> = master_decrypt("postgres__user.shop.password")?;
    let password: String = String::from_utf8(password)
        .map_err(|e| ShopError::from_error_default(Box::new(e)))?;
    let pool_result: Result<Pool<Postgres>, Error> = PgPoolOptions::new()
        .max_connections(16)
        .connect(&format!("postgres://shop:{}@localhost:5432/shop", password))
        .await;

    match pool_result {
        Ok(pool) => Ok(pool),
        Err(error) => Err(ShopError::from(error)),
    }
}
