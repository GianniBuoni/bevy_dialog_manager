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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStruct {
        script: TomlNodeMap,
    }

    fn line(s: &str) -> Line {
        Line(s.into())
    }

    fn text_line() -> TextLine {
        TextLine {
            line: line("line"),
            id: 0,
            weight: 1.,
        }
    }

    fn toml_node() -> TomlNode {
        TomlNode::Talk(TomlTalk {
            text: TomlText(vec![text_line()]),
            next: Some(line("line")),
        })
    }

    fn toml_node_map(node_name: &str) -> TomlNodeMap {
        let mut map = HashMap::new();
        map.insert(line(node_name), toml_node());
        TomlNodeMap(map)
    }

    #[test]
    fn test_de() -> Result<()> {
        let mut test_cases = Vec::<(TestStruct, &str, &str)>::new();
        test_cases.push((
            TestStruct {
                script: toml_node_map("line"),
            },
            "script.line.talk = { text = [\"line\"], next = \"line\" }",
            "script.line.talk = { text = [\"line\"], next = \"line\" }",
        ));
        test_cases.push((
            TestStruct {
                script: toml_node_map("works ok"),
            },
            "script.\"works ok\".talk = { text = [\"line\"], next = \"line\" }",
            "script.\"works ok\".talk = { text = [\"line\"], next = \"line\" }",
        ));
        test_cases.push((
            TestStruct {
                script: toml_node_map("1"),
            },
            "script.1.talk = { text = [\"line\"], next = \"line\" }",
            "script.1.talk = { text = [\"line\"], next = \"line\" }",
        ));
        de_test::<TestStruct>(test_cases)
    }

    #[test]
    fn test_error_de() {
        let test_cases = vec![
            (
                "script.\"should work\".invalid = { text = [\"line\"], next = \"line\" }",
                "invalid node",
            ),
            ("script.valid.talk = \"bad node\"", "bad node"),
        ];
        de_error_test::<TestStruct>(test_cases);
    }
}
