use super::*;

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
                if let Some((node, value)) =
                    map.next_entry::<String, toml::Table>()?
                {
                    let node = TomlNode::from_toml_value(node.as_str(), value)
                        .map_err(|e| de::Error::custom(e))?;

                    Ok(node)
                } else {
                    Err(de::Error::custom("toml file contains no valid nodes"))
                }
            }
        }
        deserializer.deserialize_map(TomlNodeVisitor)
    }
}
