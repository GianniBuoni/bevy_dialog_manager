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

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize, PartialEq, Debug)]
    struct TestStruct {
        text: TextLine,
    }

    #[test]
    fn test_de() -> Result<()> {
        let mut test_cases = Vec::new();
        test_cases.push((
            TestStruct {
                text: TextLine {
                    line: Line("Oh hi, there!".into()),
                    id: 0,
                    weight: 1.,
                },
            },
            "text = \"Oh hi, there!\"",
            "string",
        ));
        test_cases.push((
            TestStruct {
                text: TextLine {
                    line: Line("This is a random line.".into()),
                    id: 0,
                    weight: 0.5,
                },
            },
            "text = {id = 0, weight = 0.5, line = \"This is a random line.\"}",
            "map",
        ));
        de_test::<TestStruct>(test_cases)
    }

    #[test]
    fn test_de_error() {
        let test_cases = vec![
            ("invalid toml", "invalid toml"),
            ("text = [1, 2, 3]", "invalid type"),
            (
                "text = [1, 2, \"three\"]",
                "invalid type: mixed strings and int",
            ),
        ];
        de_error_test::<TestStruct>(test_cases)
    }
}
