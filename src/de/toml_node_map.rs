use bevy::platform::collections::HashMap;

use super::*;

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
                let mut node_map = HashMap::new();
                while let Some((k, v)) =
                    map.next_entry::<String, toml::Table>()?
                {
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
