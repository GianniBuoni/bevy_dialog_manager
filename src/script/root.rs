use serde::Deserialize;

use super::*;

pub mod prelude {
    pub use super::RootNode;
}

/// Defines the core checks, variables, and actors of a dialog tree
/// The asset loader should parse this before handling the rest of the nodes.
#[derive(Reflect, Debug, Clone, Deserialize)]
pub struct RootNode {
    pub start_node: Line,
}
