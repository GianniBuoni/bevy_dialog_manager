use bevy::{app::PluginGroupBuilder, prelude::*};

mod de;
mod errors;
mod loaders;
mod resources;
mod script;

pub mod prelude {
    pub use super::DialogManagerPlugin;
    pub use super::errors::prelude::*;
    pub use super::resources::prelude::*;
    pub use super::script::prelude::*;
}

/// Plugin group adding all of the library's sub-plugins.
/// All the plugins work together to load dialog assets into the game.
pub struct DialogManagerPlugin;

impl PluginGroup for DialogManagerPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(loaders::DialogLoaderPlugin)
            .add(resources::DialogRescourcePlugin)
    }
}
