pub mod db;
pub mod server;

mod category;

pub mod env {
	pub fn load_env() -> Result<(), std::io::Error> {
		match dotenvy::dotenv() {
			Ok(_) => Ok(()),
			Err(error) => Err(std::io::Error::new(std::io::ErrorKind::Other, error))?,
		}
	}
}
