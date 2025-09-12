use thiserror::Error;

pub mod prelude {
    pub use super::{DialogLoaderError, ScriptValidationError};
}

/// Error type that wraps around the deserilaization
/// errors possible during asset loading.
#[derive(Debug, Error)]
pub enum DialogLoaderError {
    /// A standard io error.
    #[error("I/O error. {0}")]
    Io(#[from] std::io::Error),
    /// Indicates that passed in file is not valid TOML
    /// or does not fit the available schemas for deserialization.
    #[error("Could not parse TOML. {0}")]
    Toml(#[from] toml::de::Error),
}

/// Error type for data validation issues. Idicates that the TOML is valid,
/// and the passed in file fits the schema for asset deserilaization, BUT
/// there is an issue with the data values.
#[derive(Debug, Error)]
pub enum ScriptValidationError {
    /// Issue with probablility weights for line of the same id. 1. == 100%,
    /// and there is an issue if the total probability if all lines to choose
    /// from doesn't add up to 1.
    #[error(
        "Invalid text line weights. TextLine with id {id} totals to probability weight != 1. Last line processed: \"{text_line}\"."
    )]
    TextWeight { text_line: String, id: usize },
    /// ID's can be ommited in the TOML, and should be automatically assigned
    /// during deserilalization. On the chance they aren't, this error variant
    /// catches that.
    #[error("TextLine: \"{text_line}\" has no assigned id")]
    UnassignedId { text_line: String },
    /// Catches an passed in ID's that are valid TOML integers (i64),
    /// but not valid for the a TextLine's ID (usize).
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
