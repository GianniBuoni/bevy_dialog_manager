use bevy::prelude::*;

use crate::prelude::*;

mod talk;

pub struct NodesPlugin;

impl Plugin for NodesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DialogHandle>();
        app.register_type::<TextLine>();

        app.add_plugins(talk::plugin);
    }
}

/// Trait defining behavior of all dialog Nodes
pub trait Node {
    fn next(&self);
}

/// Identifier for node types. Includes a usize and string identifiers
/// for various comparison needs.
#[derive(Debug, Clone, Reflect)]
pub struct DialogHandle {
    pub id: usize,
    pub str: Line,
}

/// Basic element of a dialog node.
#[derive(Debug, Clone, Reflect)]
struct TextLine {
    line: Line,
    id: usize,
    weight: f32,
}

impl Default for TextLine {
    fn default() -> Self {
        Self {
            line: Line::default(),
            id: 0,
            weight: 1.,
        }
    }
}
