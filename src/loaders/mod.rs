use bevy::{asset::AssetLoader, platform::collections::HashMap, prelude::*};
use toml::Table;

use crate::prelude::*;

mod de;
mod toml_types;

pub mod prelude {
    pub(crate) use super::toml_types::predlude::*;
}

pub struct DialogLoaderPlugin;

impl Plugin for DialogLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<DialogAssetLoader>();
        app.init_asset::<DialogScript>();
    }
}

#[derive(Default)]
pub struct DialogAssetLoader;

impl AssetLoader for DialogAssetLoader {
    type Asset = DialogScript;
    type Settings = ();
    type Error = DialogLoaderError;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        _load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        // try to read the file
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        // deserialize the bytes into TOML
        let raw_script =
            toml::from_slice::<TomlScript>(&bytes).map_err(|e| {
                let debug = match toml::from_slice::<Table>(&bytes) {
                    Ok(d) => d,
                    Err(e) => {
                        return e;
                    }
                };
                dbg!(debug);
                e
            })?;
        dbg!(raw_script);

        // parse the TomlScript into DialogScript

        // placeholder
        Ok(DialogScript(HashMap::new()))
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }
}
