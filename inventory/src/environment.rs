use crate::error::ShopError;
use log::{LevelFilter, SetLoggerError};
use std::backtrace::Backtrace;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::string::ToString;
use std::sync::LazyLock;
use std::{env, fs};

static RUNTIME_ENVIRONMENT_DEFAULT: LazyLock<RuntimeEnvironment> = LazyLock::new(||
    RuntimeEnvironment::from_env().unwrap_or(RuntimeEnvironment::Local)
);
static CARGO_MANIFEST_DIR: LazyLock<String> = LazyLock::new(||
    env::var("CARGO_MANIFEST_DIR").unwrap_or("/dev/null".to_string())
);

const VOLATILE_DIRECTORY_NAME: &str = "volatile";
const IMAGES_DIRECTORY_NAME: &str = "images";

#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeEnvironment {
    Local = 0,
    Stage,
    Production,
}

impl RuntimeEnvironment {
    pub fn from_env() -> Result<RuntimeEnvironment, ShopError> {
        RuntimeEnvironment::try_from(
            env::var("RUNTIME_ENVIRONMENT").unwrap_or(String::from("local"))
        )
    }
}

impl Default for RuntimeEnvironment {
    fn default() -> RuntimeEnvironment {
        RUNTIME_ENVIRONMENT_DEFAULT.deref().clone()
    }
}

impl TryFrom<String> for RuntimeEnvironment {
    type Error = ShopError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "local" => Ok(Self::Local),
            "stage" => Ok(Self::Stage),
            "production" => Ok(Self::Production),
            _ => Err(ShopError::new(&format!("Error parsing runtime environment [{}]", value)))
        }
    }
}

pub fn load_env() -> Result<(), std::io::Error> {
    dotenvy::dotenv()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(())
}

pub fn init_logger() -> Result<(), SetLoggerError> {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .filter_module("actix_server", LevelFilter::Debug)
        .filter_module("actix_web::middleware::logger", LevelFilter::Warn)
        .format_source_path(true)
        .try_init()
}

/// Capture a backtrace, ignoring RUST_BACKTRACE and RUST_LIB_BACKTRACE environment variables in non-production environments.
pub fn capture_backtrace() -> Backtrace {
    match RuntimeEnvironment::default() {
        RuntimeEnvironment::Local | RuntimeEnvironment::Stage => Backtrace::force_capture(),
        RuntimeEnvironment::Production => Backtrace::capture(),
    }
}

pub fn images_directory_path() -> Result<PathBuf, ShopError> {
    let path: PathBuf = if RuntimeEnvironment::default() == RuntimeEnvironment::Local {
        /* For local development, it is expected that the server is running on the host system.
            This configuration may not work if running locally inside a container. */
        let manifest_path: &Path = Path::new(&*CARGO_MANIFEST_DIR);
        let workspace_path: &Path = manifest_path.parent()
            .ok_or_else(|| ShopError::default())?;
        let images_path: PathBuf = workspace_path.join(VOLATILE_DIRECTORY_NAME).join(IMAGES_DIRECTORY_NAME);

        let images_directory_exists: bool = fs::exists(&images_path)
            .map_err(|e| ShopError::from_error_default(Box::new(e)))?;
        if !images_directory_exists {
            fs::create_dir_all(&images_path)
                .map_err(|e| ShopError::from_error_default(Box::new(e)))?;
        }

        images_path
    } else {
        // Container volume
        Path::new("/").join(IMAGES_DIRECTORY_NAME).to_path_buf()
    };
    Ok(path)
}
