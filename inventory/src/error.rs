use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ShopError {}

impl Display for ShopError {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "InventoryError")
	}
}

impl Error for ShopError {}
