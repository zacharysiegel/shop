use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ShopError {
    pub message: String,
    pub sub_error: Option<Box<dyn Error>>,
}

impl ShopError {
    pub fn new(message: &str) -> ShopError {
        ShopError {
            message: format!("Error: {}", message),
            sub_error: None,
        }
    }

    pub fn from_error(message: &str, error: Box<dyn Error>) -> ShopError {
        ShopError {
            message: format!("Error: {}", message),
            sub_error: Some(error),
        }
    }
}

impl Display for ShopError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ShopError [{}]", self.message)
    }
}

impl Error for ShopError {}

impl Default for ShopError {
    fn default() -> Self {
        Self::new("unspecified")
    }
}

impl From<sqlx::Error> for ShopError {
    fn from(error: sqlx::Error) -> Self {
        Self::from_error(&format!("{:#}", error), Box::new(error))
    }
}
