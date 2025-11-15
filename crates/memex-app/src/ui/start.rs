use gpui::{App, Entity, div, prelude::*};
use gpui_component::v_flex;

pub struct Start;

impl Start {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self)
    }
}

impl Render for Start {
    fn render(&mut self, _window: &mut gpui::Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .justify_center()
            .items_center()
            .child(div().text_3xl().child("読み込み中..."))
    }
}
