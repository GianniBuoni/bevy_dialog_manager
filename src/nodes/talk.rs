use super::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<TalkNode>();
}

#[derive(Debug, Clone, Reflect)]
pub struct TalkNode {
    pub text: std::sync::Arc<[TextLine]>,
    pub next: Option<DialogHandle>,
}

impl Node for TalkNode {
    fn next(&self) {
        let Some(next) = self.next.as_ref() else {
            return;
        };
        info!("{next:?}")
    }
}
