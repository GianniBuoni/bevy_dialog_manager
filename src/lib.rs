use bevy::{app::PluginGroupBuilder, prelude::*};

mod de;
mod errors;
mod loaders;
mod script;

pub mod prelude {
    pub use super::DialogManagerPlugin;
    pub use super::errors::prelude::*;
    pub use super::script::prelude::*;
}

pub struct DialogManagerPlugin;

impl PluginGroup for DialogManagerPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(script::NodesPlugin)
            .add(loaders::DialogLoaderPlugin)
    }
}
