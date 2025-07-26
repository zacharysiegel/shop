use crate::decrypt::master_decrypt;
use crate::environment::RuntimeEnvironment;
use crate::error::ShopError;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub async fn sqlx_connect() -> Result<Pool<Postgres>, ShopError> {
    let host: String = get_db_host();
    let password: String = get_db_password()?;

    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(16)
        .connect(&format!("postgres://shop:{}@{}/shop", password, host))
        .await
        .map_err(|e| ShopError::from(e))?;
    Ok(pool)
}

fn get_db_password() -> Result<String, ShopError> {
    let runtime_environment: RuntimeEnvironment = RuntimeEnvironment::default();
    let password_key: String = format!("postgres__user.shop.password.{}", runtime_environment.to_string());

    let password: Vec<u8> = master_decrypt(&password_key)?;
    let password: String = String::from_utf8(password)
        .map_err(|e| ShopError::from_error_default(Box::new(e)))?;
    Ok(password)
}

fn get_db_host() -> String {
    let runtime_environment: RuntimeEnvironment = RuntimeEnvironment::default();
    match runtime_environment {
        // Local development does not run the Rust applications inside containers
        RuntimeEnvironment::Local => "localhost:5432".to_string(),
        RuntimeEnvironment::Stage | RuntimeEnvironment::Production => "postgres:5432".to_string(),
    }
}
