use crate::error::ShopError;
use log::{LevelFilter, SetLoggerError};
use std::backtrace::Backtrace;
use std::env;
use std::ops::Deref;
use std::sync::LazyLock;

static RUNTIME_ENVIRONMENT_DEFAULT: LazyLock<RuntimeEnvironment> = LazyLock::new(||
    RuntimeEnvironment::from_env().unwrap_or(RuntimeEnvironment::Local)
);

#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeEnvironment {
    Local = 0,
    Stage,
    Production,
}

impl RuntimeEnvironment {
    pub fn from_env() -> Result<RuntimeEnvironment, ShopError> {
        RuntimeEnvironment::try_from(
            env::var("RUNTIME_ENVIRONMENT").unwrap_or(String::from("local"))
        )
    }
}

impl Default for RuntimeEnvironment {
    fn default() -> RuntimeEnvironment {
        RUNTIME_ENVIRONMENT_DEFAULT.deref().clone()
    }
}

impl TryFrom<String> for RuntimeEnvironment {
    type Error = ShopError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "local" => Ok(Self::Local),
            "stage" => Ok(Self::Stage),
            "production" => Ok(Self::Production),
            _ => Err(ShopError::new(&format!("Error parsing runtime environment [{}]", value)))
        }
    }
}

pub fn load_env() -> Result<(), std::io::Error> {
    dotenvy::dotenv()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(())
}

pub fn init_logger() -> Result<(), SetLoggerError> {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .filter_module("actix_server", LevelFilter::Debug)
        .filter_module("actix_web::middleware::logger", LevelFilter::Info)
        .format_source_path(true)
        .try_init()
}

/// Capture a backtrace, ignoring RUST_BACKTRACE and RUST_LIB_BACKTRACE environment variables in non-production environments.
pub fn capture_backtrace() -> Backtrace {
    match RuntimeEnvironment::default() {
        RuntimeEnvironment::Local | RuntimeEnvironment::Stage => Backtrace::force_capture(),
        RuntimeEnvironment::Production => Backtrace::capture(),
    }
}
