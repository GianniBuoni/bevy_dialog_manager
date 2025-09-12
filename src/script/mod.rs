//! When writing a new node try to keep new nodes as small as possible
//! and have simple responsibilities.
use bevy::{platform::collections::HashMap, prelude::*};
use serde::Deserialize;

use crate::prelude::*;

mod components;
mod nodes;
mod root;
mod talk;

/// Required. Plugin registers types related to nodes and fields
/// that make up all the node types.
pub struct NodesPlugin;

pub mod prelude {
    pub use super::DialogScript;
    pub(crate) use super::components::prelude::*;
    pub use super::nodes::prelude::*;
    pub use super::root::prelude::*;
    pub use super::talk::prelude::*;
    pub(crate) use super::{TomlNodeMap, TomlScript};
}

impl Plugin for NodesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DialogScript>();

        app.add_plugins(components::plugin);
        app.add_plugins(talk::plugin);
        app.add_plugins(root::plugin);
    }
}

/// Main Asset for the loader. Contains a root node
/// that provides metadata about the entire conversation tree,
/// and a hashmap of Line -> DialogNode pairs.
#[derive(Debug, Asset, Reflect)]
pub struct DialogScript {
    pub root: RootNode,
    pub script: HashMap<Line, DialogNode>,
}

/// Toml representation of a Dialog script.
#[derive(Debug, Deserialize)]
pub(crate) struct TomlScript {
    pub(crate) root: RootNode,
    pub(crate) script: TomlNodeMap,
}

/// Newtype wrapper for a Line -> TomlNode hashmap.
/// Used only during deserialization. The asset loader will
/// call the conversion that handles unwrapping the hashmap.
#[derive(Debug, Default, PartialEq)]
pub(crate) struct TomlNodeMap(pub(crate) HashMap<Line, TomlNode>);

impl From<TomlScript> for DialogScript {
    fn from(value: TomlScript) -> Self {
        let script = value
            .script
            .0
            .into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect();

        Self {
            root: value.root,
            script,
        }
    }
}
