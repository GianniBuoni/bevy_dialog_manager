use thiserror::Error;

pub mod prelude {
    pub use super::DialogLoaderError;
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum DialogLoaderError {
    /// A standard io error
    #[error("I/O error. {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse TOML. {0}")]
    Toml(#[from] toml::de::Error),
}
