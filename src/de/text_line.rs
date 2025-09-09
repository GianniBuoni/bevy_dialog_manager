use bevy::platform::collections::HashMap;

use super::*;

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
                let mut line_map = HashMap::new();
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
