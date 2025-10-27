use serde::de::Error;
use strum::Display;

use super::*;

pub mod prelude {
    pub use super::DialogNode;
    pub(crate) use super::TomlNode;
}

/// Enumeration of all possible actions in a script.
#[derive(Debug, Reflect)]
pub enum DialogNode {
    Talk(TalkNode),
}

impl DialogNode {
    pub fn next(&self) -> Option<Line> {
        match self {
            Self::Talk(node) => node.next.clone(),
        }
    }
}

#[derive(Debug, Display, PartialEq)]
pub(crate) enum TomlNode {
    Talk(TomlTalk),
}

impl From<TomlNode> for DialogNode {
    fn from(value: TomlNode) -> Self {
        match value {
            TomlNode::Talk(t) => Self::Talk(t.into()),
        }
    }
}

impl TomlNode {
    pub fn from_toml_value(
        node_type: &str,
        value: toml::Table,
    ) -> Result<Self, DialogLoaderError> {
        let node = match node_type {
            "talk" => TomlNode::Talk(TomlTalk::deserialize(value)?),
            _ => {
                return Err(DialogLoaderError::Toml(toml::de::Error::custom(
                    "unhandled node type",
                )));
            }
        };
        Ok(node)
    }
}
