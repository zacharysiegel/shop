pub fn load_env() -> Result<(), std::io::Error> {
    match dotenvy::dotenv() {
        Ok(_) => Ok(()),
        Err(error) => Err(std::io::Error::new(std::io::ErrorKind::Other, error))?,
    }
}
