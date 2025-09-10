use bevy::platform::collections::HashSet;

use super::*;

pub mod prelude {
    pub use super::TalkNode;
    pub(crate) use super::{TomlTalk, TomlText};
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<TalkNode>();
}

/// The most basic dialog node. Contains text for display.
#[derive(Reflect, Debug, Clone)]
pub struct TalkNode {
    pub text: std::sync::Arc<[TextLine]>,
    pub next: Option<Line>,
}

#[derive(Debug, Default, Deserialize, PartialEq)]
pub(crate) struct TomlTalk {
    pub(crate) text: TomlText,
    pub(crate) next: Option<Line>,
}

#[derive(Debug, Default, PartialEq)]
pub(crate) struct TomlText(pub Vec<TextLine>);

impl From<TomlTalk> for TalkNode {
    fn from(value: TomlTalk) -> Self {
        let text = value.text.0.into();
        Self {
            text,
            next: value.next,
        }
    }
}

impl<'de> TomlText {
    /// Method to validate probability weights during deserialization
    pub fn validate_weights(&'de self) -> Result<(), ScriptValidationError> {
        // make a hash set of ids
        let ids = self.0.iter().map(|f| f.id).collect::<HashSet<usize>>();

        ids.iter().try_for_each(|id| {
            let (text_line, weight): (&'de str, f32) = self
                .0
                .iter()
                .filter(|f| f.id == *id)
                .fold(("", 0.), |mut acc, f| {
                    acc.0 = f.line.0.as_ref();
                    acc.1 += f.weight;
                    acc
                });
            if weight != 1. {
                return Err(ScriptValidationError::text_weight(text_line, *id));
            }
            Ok::<(), ScriptValidationError>(())
        })?;

        Ok(())
    }
}
