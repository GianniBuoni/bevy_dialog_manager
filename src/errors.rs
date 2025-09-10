use thiserror::Error;

pub mod prelude {
    pub use super::{DialogLoaderError, ScriptValidationError};
}

#[derive(Debug, Error)]
pub enum DialogLoaderError {
    /// A standard io error
    #[error("I/O error. {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse TOML. {0}")]
    Toml(#[from] toml::de::Error),
    #[error("Coudn't convert TOML to a Dialog asset: {message}")]
    TryFrom { message: String },
}

#[derive(Debug, Error)]
pub enum ScriptValidationError {
    #[error(
        "Invalid text line weights. TextLine with id {id} totals to probability weight != 1. Last line processed: \"{text_line}\"."
    )]
    TextWeight { text_line: String, id: usize },
    #[error("TextLine: \"{text_line}\" has no assigned id")]
    UnassignedId { text_line: String },
    #[error(
        "TextLine \"{text_line}\" has an invalid id: {id}. Only unsigned itegers are allowed."
    )]
    InvalidId { text_line: String, id: i64 },
}

impl ScriptValidationError {
    pub fn text_weight(text_line: &str, id: usize) -> Self {
        Self::TextWeight {
            text_line: text_line.into(),
            id,
        }
    }
}
