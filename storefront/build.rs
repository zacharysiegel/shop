use static_files::resource;
use std::{env, path};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let generated_filename = path::Path::new(&out_dir).join("generated.rs");
    resource::generate_resources_mapping("./static", None, generated_filename)?;

    Ok(())
}
