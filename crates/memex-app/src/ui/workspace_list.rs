use gpui::{App, Window, div, prelude::*};
use gpui_component::{ActiveTheme, Icon, Sizable, v_flex};
use memex_backend::{SystemContext, data::WorkspaceIconData};
use uuid::Uuid;

pub struct WorkspaceList;

impl WorkspaceList {
    pub fn new() -> Self {
        Self
    }
}

impl Render for WorkspaceList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        let system = cx.global::<SystemContext>().clone();
        let manager = system.workspace_manager().lock().unwrap();

        v_flex()
            .id("workspace-list")
            .gap_2()
            .items_center()
            // Home workspace
            .child(
                v_flex()
                    .justify_center()
                    .items_center()
                    .size_12()
                    .p_4()
                    .rounded_2xl()
                    .text_color(cx.theme().accent_foreground)
                    .when_else(
                        manager.selected() == manager.home(),
                        |this| this.bg(cx.theme().accent),
                        |this| this.bg(cx.theme().accent.alpha(0.4)),
                    )
                    .child(Icon::empty().path("icons/house.svg").large()),
            )
            .child(div().w_3_4().border_1().border_color(cx.theme().border))
            // User workspaces
            .children(manager.order().iter().enumerate().map(|(ix, id)| {
                let list = manager.list_metadata();
                let metadata = list.get(&id).unwrap();

                v_flex()
                    .id(ix)
                    .size_12()
                    .when(manager.selected() == metadata.id(), |this| {
                        this.bg(cx.theme().primary.alpha(0.2))
                    })
                    .rounded_xl()
                    .justify_center()
                    .text_center()
                    .text_3xl()
                    .child(WorkspaceIcon(*id))
            }))
    }
}

#[derive(IntoElement)]
struct WorkspaceIcon(Uuid);

impl RenderOnce for WorkspaceIcon {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let system = cx.global::<SystemContext>();
        let manager = system.workspace_manager().lock().unwrap();
        let metadata = manager.list_metadata().get(&self.0).unwrap();

        match metadata.icon() {
            WorkspaceIconData::Default => v_flex()
                .justify_center()
                .items_center()
                .child(Icon::empty().path("icons/album.svg")),
            WorkspaceIconData::Emoji(emoji) => v_flex()
                .justify_center()
                .items_center()
                .text_xl()
                .child(emoji.to_owned()),
            WorkspaceIconData::Text(text) => v_flex()
                .justify_center()
                .items_center()
                .child(text.to_owned()),
            _ => v_flex().child("😳"),
        }
    }
}
