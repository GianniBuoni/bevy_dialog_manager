use super::*;

pub mod prelude {
    pub use super::TalkNode;
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

impl From<TomlTalk> for TalkNode {
    fn from(value: TomlTalk) -> Self {
        let text = value.text.0.into();
        Self {
            text,
            next: value.next,
        }
    }
}

impl Node for TalkNode {
    fn next(&self) {
        let Some(next) = self.next.as_ref() else {
            return;
        };
        info!("{next:?}")
    }
}
