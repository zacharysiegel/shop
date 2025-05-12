use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ShopError {
    pub message: String,
}

impl ShopError {
    pub fn new(message: &str) -> ShopError {
        ShopError {
            message: format!("Error: {}", message)
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
