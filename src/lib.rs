use bevy::{app::PluginGroupBuilder, prelude::*};

mod nodes;

pub mod prelude {
    pub use super::DialogManagerPlugin;
    pub(crate) use super::Line;
}

/// type alias for [`Arc<str>`]
pub(crate) type Line = std::sync::Arc<str>;

pub struct DialogManagerPlugin;

impl PluginGroup for DialogManagerPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(nodes::NodesPlugin)
    }
}
