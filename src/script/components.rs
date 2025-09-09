//! All structs that make up the feilds of Dialog Nodes
use super::*;

pub mod prelude {
    pub use super::Line;
    pub(crate) use super::TextLine;
}

/// plugin for this module
pub(super) fn plugin(app: &mut App) {
    app.register_type::<TextLine>();
    app.register_type::<Line>();
}

/// newtype alias for [`Arc<str>`]
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
