use indexmap::IndexMap;
use serde::{
    Deserialize,
    de::{self, Visitor},
};
use toml::Table;

use crate::prelude::*;

impl<'de> Deserialize<'de> for TomlNodeMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TomlNodeMapVisitor;

        impl<'de> Visitor<'de> for TomlNodeMapVisitor {
            type Value = TomlNodeMap;

            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                write!(formatter, "a map of node id's -> node maps")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut node_map = IndexMap::new();
                while let Some((k, v)) = map.next_entry::<String, Table>()? {
                    let line = Line(k.into());
                    let node = TomlNode::deserialize(v)
                        .map_err(|e| de::Error::custom(e))?;
                    node_map.insert(line, node);
                }
                Ok(TomlNodeMap(node_map))
            }
        }
        deserializer.deserialize_map(TomlNodeMapVisitor)
    }
}

impl<'de> Deserialize<'de> for TomlNode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TomlNodeVisitor;

        impl<'de> Visitor<'de> for TomlNodeVisitor {
            type Value = TomlNode;

            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                write!(
                    formatter,
                    "a map of a node types -> node structs as a map"
                )
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let Some((node, value)) = map.next_entry::<String, Table>()?
                else {
                    return Ok(TomlNode::None);
                };
                let node = TomlNode::from_toml_value(node.as_str(), value)
                    .map_err(|e| de::Error::custom(e))?;

                Ok(node)
            }
        }
        deserializer.deserialize_map(TomlNodeVisitor)
    }
}

impl<'de> Deserialize<'de> for Line {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct LineVisitor;

        impl<'de> Visitor<'de> for LineVisitor {
            type Value = Line;

            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                write!(formatter, "string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Line(v.into()))
            }
        }
        deserializer.deserialize_string(LineVisitor)
    }
}

impl<'de> Deserialize<'de> for TextLine {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct TextLineVisitor;

        impl<'de> Visitor<'de> for TextLineVisitor {
            type Value = TextLine;

            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                write!(formatter, "string or map")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut line_map = IndexMap::new();
                while let Some((k, v)) =
                    map.next_entry::<String, toml::Value>()?
                {
                    line_map.insert(k, v);
                }
                let line = line_map
                    .get("line")
                    .unwrap_or(&toml::Value::String(String::default()))
                    .as_str()
                    .unwrap_or_default()
                    .into();

                let id = line_map
                    .get("id")
                    .unwrap_or(&toml::Value::Integer(0))
                    .as_integer()
                    .unwrap_or_default() as usize;

                let weight = line_map
                    .get("weight")
                    .unwrap_or(&toml::Value::Float(1.))
                    .as_float()
                    .unwrap_or_default() as f32;

                Ok(TextLine { line, id, weight })
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let line = Line(v.into());
                Ok(TextLine {
                    line,
                    id: 0,
                    weight: 1.,
                })
            }
        }
        deserializer.deserialize_any(TextLineVisitor)
    }
}
