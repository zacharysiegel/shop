use rand::RngCore;
use rand::prelude::ThreadRng;
use uuid::Uuid;

pub mod db;
pub mod server;

mod category;
mod inventory_location;
mod item;
mod product;

pub mod env {
	pub fn load_env() -> Result<(), std::io::Error> {
		match dotenvy::dotenv() {
			Ok(_) => Ok(()),
			Err(error) => Err(std::io::Error::new(std::io::ErrorKind::Other, error))?,
		}
	}
}

pub fn random_uuid() -> Uuid {
	let mut rng: ThreadRng = rand::rng();
	let mut random_bytes: [u8; 128 >> 3] = [0; 128 >> 3]; // 128 bits
	rng.fill_bytes(&mut random_bytes); // ThreadRng::fill_bytes never panics

	assert_eq!(random_bytes.len(), 16);
	Uuid::from_slice(&random_bytes[..]).unwrap() // Err is only returned for non 16 byte length
}

/// Standard mappings between structs at the service-level and api-level
pub trait InventoryEntity {
	type Serializable;

	fn to_serial(&self) -> Self::Serializable;
	fn from_serial(serializable: &Self::Serializable) -> Self;
}
