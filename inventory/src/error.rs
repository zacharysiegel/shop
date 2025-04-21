use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct InventoryError {}

impl Display for InventoryError {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "InventoryError")
	}
}

impl Error for InventoryError {}
