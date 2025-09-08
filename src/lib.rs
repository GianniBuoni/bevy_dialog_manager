use bevy::{app::PluginGroupBuilder, prelude::*};

mod errors;
mod loaders;
mod nodes;

pub mod prelude {
    pub use super::DialogManagerPlugin;
    pub use super::Line;
    pub use super::errors::prelude::*;
    pub(crate) use super::loaders::prelude::*;
    pub use super::nodes::prelude::*;
}

/// newtype alias for [`Arc<str>`]
#[derive(Debug, Default, Clone, Reflect, Hash, PartialEq, Eq)]
pub struct Line(std::sync::Arc<str>);

impl From<&str> for Line {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

pub struct DialogManagerPlugin;

impl PluginGroup for DialogManagerPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(nodes::NodesPlugin)
            .add(loaders::DialogLoaderPlugin)
    }
}
