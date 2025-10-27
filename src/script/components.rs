//! All structs that make up the feilds of Dialog Nodes
use super::*;

pub mod prelude {
    pub use super::Line;
    pub(crate) use super::{IdAssigned, TextLine, TomlTextLine};
}

/// Newtype alias for [`Arc<str>`]
#[derive(Debug, Default, Clone, Reflect, Hash, PartialEq, Eq)]
pub struct Line(pub std::sync::Arc<str>);

impl From<&str> for Line {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

/// Basic element of a dialog node.
#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct TextLine {
    pub line: Line,
    pub id: usize,
    pub weight: f32,
}

/// Unprocessed toml version of [`TextLine`]
#[derive(Debug, PartialEq)]
pub struct TomlTextLine {
    pub line: Line,
    pub id: IdAssigned,
    pub weight: f32,
}

/// Enum for deserializing a TomlTextLine id's.
/// Handles thes possibility that the id has not been
/// assigned in the TOML file.
#[derive(Debug, PartialEq)]
pub enum IdAssigned {
    Unassigned,
    Assigned(usize),
}

impl TryFrom<TomlTextLine> for TextLine {
    type Error = ScriptValidationError;

    fn try_from(value: TomlTextLine) -> std::result::Result<Self, Self::Error> {
        let IdAssigned::Assigned(id) = value.id else {
            return Err(ScriptValidationError::UnassignedId {
                text_line: value.line.0.to_string(),
            });
        };

        Ok(Self {
            line: value.line,
            id,
            weight: value.weight,
        })
    }
}
