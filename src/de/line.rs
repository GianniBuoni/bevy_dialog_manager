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
