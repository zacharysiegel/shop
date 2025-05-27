use std::backtrace::{Backtrace, BacktraceStatus};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ShopError {
    pub message: String,
    pub sub_error: Option<Box<dyn Error>>,
    pub backtrace: Backtrace,
}

impl ShopError {
    pub fn new(message: &str) -> ShopError {
        Self::_new(message, None)
    }

    pub fn from_error(message: &str, error: Box<dyn Error>) -> ShopError {
        Self::_new(message, Some(error))
    }

    fn _new(message: &str, error: Option<Box<dyn Error>>) -> ShopError {
        let backtrace: Backtrace = crate::environment::capture_backtrace();
        let shop_error = ShopError {
            message: format!("Error: {}", message),
            sub_error: error,
            backtrace,
        };
        shop_error
    }
}

impl Display for ShopError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ShopError [{}]", self.message)?;
        match self.backtrace.status() {
            BacktraceStatus::Unsupported | BacktraceStatus::Disabled => Ok(()),
            BacktraceStatus::Captured => write!(f, "\n{}", self.backtrace),
            _ => Ok(()),
        }
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
