//! All structs that make up the feilds of Dialog Nodes
use super::*;

pub mod prelude {
    pub(crate) use super::TextLine;
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<TextLine>();
}

/// Basic element of a dialog node.
#[derive(Debug, Clone, Reflect)]
pub struct TextLine {
    pub line: Line,
    pub id: usize,
    pub weight: f32,
}
