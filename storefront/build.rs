#![allow(unused)]

use static_files::resource;
use std::env;
use std::fmt::{Debug, Display, Formatter};
use std::path::Path;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir_str: String = env::var("OUT_DIR")?;
    let out_dir: &Path = Path::new(&out_dir_str);

    static_files(out_dir)?;

    Ok(())
}

fn static_files(out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let generated_filename = out_dir.join("public.rs");
    resource::generate_resources_mapping("./static", None, generated_filename)?;
    Ok(())
}

#[derive(Debug)]
struct BuildError {
    message: String,
}

impl BuildError {
    pub fn boxed(message: &str) -> Box<BuildError> {
        Box::new(BuildError {
            message: message.to_string(),
        })
    }
}

impl Display for BuildError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Build error: {}", self.message)
    }
}

impl std::error::Error for BuildError {}
