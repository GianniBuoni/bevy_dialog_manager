//! Module for all the intermediate types for asset loading
//! TomlTypes are what's serialized diretly from the TOML types

use bevy::platform::collections::HashMap;
use serde::Deserialize;
use strum::EnumString;

use crate::prelude::*;

pub mod predlude {
    pub(crate) use super::{
        TomlLine, TomlNode, TomlNodeMap, TomlScript, TomlTalk,
    };
}

#[derive(Debug, Deserialize)]
pub(crate) struct TomlScript {
    root: TomlRoot,
    script: TomlNodeMap,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TomlRoot {
    start_node: String,
}

#[derive(Debug, Default)]
pub(crate) struct TomlNodeMap(pub(crate) HashMap<String, TomlNode>);

#[derive(Debug, Default, EnumString)]
pub(crate) enum TomlNode {
    #[default]
    None,
    #[strum(serialize = "talk")]
    Talk(TomlTalk),
}

#[derive(Debug, Default, Deserialize)]
pub(crate) struct TomlTalk {
    text: TomlText,
    next: Option<Line>,
}

#[derive(Debug, Default, Deserialize)]
pub(crate) struct TomlText(Vec<TomlLine>);

#[derive(Debug, Default)]
pub(crate) struct TomlLine {
    pub(crate) line: Line,
    pub(crate) id: usize,
    pub(crate) weight: f32,
}
