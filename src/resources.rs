use bevy::prelude::*;

use crate::prelude::*;

pub mod prelude {
    pub use super::CurrentDialogNode;
}

/// Optional. Plugin for the library's provided Bevy components and resources.
pub struct DialogRescourcePlugin;

/// Resrource keeps track of a current dialog tree using an
/// asset handle.
#[derive(Reflect, Resource, Default)]
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
