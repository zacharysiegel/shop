use crate::error::ShopError;
use std::env;

#[derive(Debug, PartialEq)]
pub enum RuntimeEnvironment {
    Local = 0,
    Stage,
    Production,
}

impl RuntimeEnvironment {
    pub fn from_env() -> Result<RuntimeEnvironment, ShopError> {
        RuntimeEnvironment::try_from(env::var("RUNTIME_ENVIRONMENT").unwrap_or(String::new()))
    }
}

impl Default for RuntimeEnvironment {
    fn default() -> RuntimeEnvironment {
        RuntimeEnvironment::from_env().unwrap_or(RuntimeEnvironment::Local)
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
    match dotenvy::dotenv() {
        Ok(_) => Ok(()),
        Err(error) => Err(std::io::Error::new(std::io::ErrorKind::Other, error))?,
    }
}