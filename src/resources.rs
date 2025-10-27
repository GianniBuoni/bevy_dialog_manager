use bevy::prelude::*;
use thiserror::Error;

use crate::prelude::*;

pub mod prelude {
    pub use super::CurrentDialogNode;
}

#[derive(Debug, Error)]
pub enum ResourceError {
    #[error("Resouce error: dialog script handle is missing.")]
    NoHandle,
    #[error("Resource error: Handle not registered in asset server.")]
    NotRegistered,
    #[error("Resource error: Handle has no nodes attached to it.")]
    NoNodes,
}

/// Optional. Plugin for the library's provided Bevy components and resources.
pub struct DialogRescourcePlugin;

/// Resrource keeps track of a current dialog tree using an
/// asset handle.
#[derive(Debug, Reflect, Resource, Default)]
pub struct CurrentDialogNode {
    pub dialog_handle: Option<Handle<DialogScript>>,
    pub current_node: Option<Line>,
    pub next: Option<Line>,
    pub char_index: usize,
}

impl Plugin for DialogRescourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentDialogNode>();
    }
}

impl CurrentDialogNode {
    pub fn get_handle(&self) -> Result<Handle<DialogScript>, ResourceError> {
        self.dialog_handle.clone().ok_or(ResourceError::NoHandle)
    }
    pub fn get_script_nodes(
        &mut self,
        asset_server: Res<Assets<DialogScript>>,
    ) -> Result<(), ResourceError> {
        let id = self.get_handle()?;
        let dialog_script =
            asset_server.get(&id).ok_or(ResourceError::NotRegistered)?;

        let start_node_key = dialog_script.root.start_node.clone();
        let start_node = dialog_script
            .script
            .get(&start_node_key)
            .ok_or(ResourceError::NoNodes)?;

        self.current_node = Some(start_node_key);
        self.next = start_node.next();
        Ok(())
    }
}
