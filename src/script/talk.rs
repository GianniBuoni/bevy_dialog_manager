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

#[derive(Debug, Default, Deserialize, PartialEq)]
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
