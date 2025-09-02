use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod prelude {
    pub use super::DialogLoaderPlugin;
}

pub struct DialogLoaderPlugin;

impl PluginGroup for DialogLoaderPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
    }
}
