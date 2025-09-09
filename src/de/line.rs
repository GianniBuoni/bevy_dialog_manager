use super::*;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStruct {
        text: Line,
    }

    #[test]
    fn test_de() -> Result<()> {
        let mut test_cases = Vec::new();
        test_cases.push((
            TestStruct {
                text: Line("This is a line".into()),
            },
            "text = \"This is a line\"",
            "string",
        ));
        test_cases.push((
            TestStruct {
                text: Line("Wow another line".into()),
            },
            "text = \"Wow another line\"",
            "string",
        ));
        de_test::<TestStruct>(test_cases)
    }

    #[test]
    fn test_de_error() {
        let test_cases = vec![
            ("invalid toml", "invalid toml"),
            ("text = [\"array\"]", "invalid type"),
        ];
        de_error_test::<TestStruct>(test_cases);
    }
}
