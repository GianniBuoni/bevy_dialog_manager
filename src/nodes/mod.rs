//! When writing a new node try to keep new nodes as small as possible
//! and have simple responsibilities.
use bevy::prelude::*;
use indexmap::IndexMap;

use crate::prelude::*;

mod components;
mod root;
mod talk;

pub struct NodesPlugin;

pub mod prelude {
    pub(crate) use super::components::prelude::*;
    pub use super::root::prelude::*;
    pub use super::talk::prelude::*;
    pub use super::{DialogNode, DialogScript};
}

impl Plugin for NodesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(components::plugin);
        app.add_plugins(talk::plugin);
        app.add_plugins(root::plugin);
    }
}

/// Main Asset for the crate
#[derive(Debug, Asset, TypePath)]
pub struct DialogScript {
    pub root: RootNode,
    pub script: IndexMap<Line, DialogNode>,
}

impl TryFrom<TomlScript> for DialogScript {
    type Error = DialogLoaderError;

    fn try_from(value: TomlScript) -> std::result::Result<Self, Self::Error> {
        let script = value.script.0.into_iter().try_fold(
            IndexMap::new(),
            |mut acc, (k, v)| {
                acc.insert(k, v.try_into()?);
                Ok::<IndexMap<Line, DialogNode>, DialogLoaderError>(acc)
            },
        )?;

        Ok(Self {
            root: value.root,
            script,
        })
    }
}

/// Enumeration of all possible actions in a script.
#[derive(Debug)]
pub enum DialogNode {
    Talk(TalkNode),
}

impl TryFrom<TomlNode> for DialogNode {
    type Error = DialogLoaderError;
    fn try_from(value: TomlNode) -> std::result::Result<Self, Self::Error> {
        match value {
            TomlNode::Talk(t) => Ok(Self::Talk(t.into())),
            _ => {
                let message = format!("unhandled node type, {value}");
                Err(DialogLoaderError::TryFrom { message })
            }
        }
    }
}

/// Trait defining behavior of all dialog Nodes
pub trait Node {
    fn next(&self);
}
