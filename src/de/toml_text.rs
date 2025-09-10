use super::*;

impl<'de> Deserialize<'de> for TomlText {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct TomlTextVisitor;

        impl<'de> Visitor<'de> for TomlTextVisitor {
            type Value = TomlText;

            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                write!(formatter, "a sequence of strings or maps")
            }

            fn visit_seq<A>(
                self,
                mut seq: A,
            ) -> std::result::Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(v) = seq.next_element::<toml::Value>()? {
                    let text_line = TomlTextLine::deserialize(v)
                        .map_err(|e| de::Error::custom(e))?;
                    values.push(text_line);
                }
                // assign id's and make TomlTextLine -> TextLine conversions
                let values = values
                    .into_iter()
                    .enumerate()
                    .map(|(i, mut f)| {
                        if f.id == IdAssigned::Unassigned {
                            f.id = IdAssigned::Assigned(i)
                        }
                        TextLine::try_from(f)
                    })
                    .collect::<Result<Vec<TextLine>, ScriptValidationError>>()
                    .map_err(|e| de::Error::custom(e))?;

                let text = TomlText(values);
                // validate probablity weights
                text.validate_weights().map_err(|e| de::Error::custom(e))?;

                Ok(text)
            }
        }
        deserializer.deserialize_seq(TomlTextVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// String based text array
    const TEST_TOML_1: &str = "text = [
    \"Oh hi, there!\",
    \"This is a new dialog asset.\",
]";

    /// Map based taxt array
    const TEST_TOML_2: &str = "text = [
    { line = \"Oh hi, there!\", id = 0, weight = 1.0 },
    { line = \"This is a new dialog asset.\", id = 1, weight = 1.0 },
]";

    /// Map with lines that have the same id
    const TEST_TOML_RANDOM: &str = "text = [
    { line = \"Oh hi, there!\", id = 0, weight = 0.5 },
    { line = \"This is a new dialog asset.\", id = 0, weight = 0.5 },
]";

    const MIXED_TYPES: &str = "text = [
    \"Oh hi, there!\",
    { line = \"Random.\", id = 1, weight = 0.5 },
    { line = \"This is a new dialog asset.\", id = 1, weight = 0.5 },
]";

    /// Map with lines that have the same id
    const TEST_TOML_INVALID_WEIGHT: &str = "text = [
    { line = \"Oh hi, there!\", id = 0, weight = 0.1 },
    { line = \"This is a new dialog asset.\", id = 0, weight = 0.5 },
]";

    const MIXED_TYPES_INVALID: &str = "text = [
    \"Oh hi, there!\",
    { line = \"Random.\", id = 0, weight = 0.5 },
    { line = \"This is a new dialog asset.\", id = 0, weight = 0.5 },
]";

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestStruct {
        text: TomlText,
    }

    fn line(s: &str) -> Line {
        Line(s.into())
    }

    fn text_line(s: &str, id: usize, weight: f32) -> TextLine {
        TextLine {
            line: line(s),
            id,
            weight,
        }
    }

    #[test]
    fn test_de_basic() -> Result<()> {
        let want = (|| {
            let mut want = Vec::<TextLine>::new();
            want.push(text_line("Oh hi, there!", 0, 1.));
            want.push(text_line("This is a new dialog asset.", 1, 1.));

            TestStruct {
                text: TomlText(want),
            }
        })();

        let got = toml::from_str::<TestStruct>(TEST_TOML_1)?;
        assert_eq!(want, got, "Test deserializing basic string values.");

        let got = toml::from_str::<TestStruct>(TEST_TOML_2)?;
        assert_eq!(want, got, "Test deserializing basic map values.");

        Ok(())
    }

    #[test]
    fn test_de_with_weights() -> Result<()> {
        let want = (|| {
            let mut want = Vec::<TextLine>::new();
            want.push(text_line("Oh hi, there!", 0, 0.5));
            want.push(text_line("This is a new dialog asset.", 0, 0.5));

            TestStruct {
                text: TomlText(want),
            }
        })();

        let got = toml::from_str::<TestStruct>(TEST_TOML_RANDOM)?;
        assert_eq!(want, got, "Test deserializing lines with the same id");

        Ok(())
    }

    #[test]
    fn test_de_mixed_text_types() -> Result<()> {
        let want = (|| {
            let mut want = Vec::<TextLine>::new();
            want.push(text_line("Oh hi, there!", 0, 1.0));
            want.push(text_line("Random.", 1, 0.5));
            want.push(text_line("This is a new dialog asset.", 1, 0.5));

            TestStruct {
                text: TomlText(want),
            }
        })();

        let got = toml::from_str::<TestStruct>(MIXED_TYPES)?;
        assert_eq!(
            want, got,
            "Test deserializing lines with mixed array of strings and maps."
        );

        Ok(())
    }

    #[test]
    fn test_de_error() {
        let test_cases = vec![
            (TEST_TOML_INVALID_WEIGHT, "Test invalid weights."),
            (
                MIXED_TYPES_INVALID,
                "Test invalid mixed set of text line types",
            ),
        ];
        de_error_test::<TestStruct>(test_cases);
    }
}
