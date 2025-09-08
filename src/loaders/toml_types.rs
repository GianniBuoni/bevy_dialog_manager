//! Module for all the intermediate types for asset loading
//! TomlTypes are what's serialized diretly from the TOML types

use indexmap::IndexMap;
use serde::Deserialize;
use strum::Display;

use crate::prelude::*;

pub mod predlude {
    pub(crate) use super::{TomlNode, TomlNodeMap, TomlScript, TomlTalk};
}

#[derive(Debug, Deserialize)]
pub(crate) struct TomlScript {
    pub(crate) root: RootNode,
    pub(crate) script: TomlNodeMap,
}

#[derive(Debug, Default)]
pub(crate) struct TomlNodeMap(pub(crate) IndexMap<Line, TomlNode>);

#[derive(Debug, Display, Default)]
pub(crate) enum TomlNode {
    #[default]
    None,
    Talk(TomlTalk),
}

impl TomlNode {
    pub fn from_toml_value(
        node_type: &str,
        value: toml::Table,
    ) -> Result<Self, DialogLoaderError> {
        let node = match node_type {
            "talk" => TomlNode::Talk(TomlTalk::deserialize(value)?),
            _ => TomlNode::None,
        };
        Ok(node)
    }
}

#[derive(Debug, Default, Deserialize)]
pub(crate) struct TomlTalk {
    pub(crate) text: TomlText,
    pub(crate) next: Option<Line>,
}

#[derive(Debug, Default, Deserialize)]
pub(crate) struct TomlText(pub Vec<TextLine>);
