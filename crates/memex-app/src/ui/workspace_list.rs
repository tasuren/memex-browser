use gpui::{AnyElement, App, Entity, MouseButton, ReadGlobal, Window, div, prelude::*};
use gpui_component::{ActiveTheme, Icon, IconName, Sizable, v_flex};
use memex_backend::{
    LayoutState, WorkspaceListState, WorkspaceState,
    data::{AppPath, WorkspaceIconData, create_workspace},
};
use uuid::Uuid;

pub struct WorkspaceList {
    state: Entity<WorkspaceListState>,
    layout_state: Entity<LayoutState>,
}

impl WorkspaceList {
    pub fn new(
        cx: &mut App,
        state: Entity<WorkspaceListState>,
        layout_state: Entity<LayoutState>,
    ) -> Entity<Self> {
        cx.new(|_| Self {
            state,
            layout_state,
        })
    }

    pub fn state(&self) -> &Entity<WorkspaceListState> {
        &self.state
    }

    pub fn render_user_workspaces(&self, cx: &mut App) -> Vec<AnyElement> {
        let mut children = Vec::new();

        for (ix, id) in self.state.read(cx).order().iter().cloned().enumerate() {
            let metadata = self.state.read(cx).list_metadata().get(&id).unwrap();
            let list = self.state.clone();

            children.push(
                v_flex()
                    .id(ix)
                    .size_12()
                    .when(self.state.read(cx).selected() == metadata.id(), |this| {
                        this.bg(cx.theme().primary.alpha(0.2))
                    })
                    .rounded_xl()
                    .justify_center()
                    .text_center()
                    .text_3xl()
                    .child(WorkspaceIcon {
                        list: list.clone(),
                        workspace_id: id,
                    })
                    .on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                        list.update(cx, |list, cx| list.open(window, cx, id));
                    })
                    .into_any_element(),
            );
        }

        children
    }
}

impl Render for WorkspaceList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        let app = self.state.read(cx);

        let element = v_flex()
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
                        app.selected() == app.home(),
                        |this| this.bg(cx.theme().accent),
                        |this| this.bg(cx.theme().accent.alpha(0.4)),
                    )
                    .child(Icon::empty().path("icons/house.svg").large())
                    .on_mouse_down(MouseButton::Left, {
                        let list = self.state.clone();

                        move |_event, window, cx| {
                            list.update(cx, |list, cx| list.open(window, cx, list.home()));
                        }
                    }),
            )
            .child(
                div()
                    .mx_1()
                    .my_2()
                    .w_3_4()
                    .h_0()
                    .border_1()
                    .border_color(cx.theme().border),
            );

        let user_workspaces = self.render_user_workspaces(cx);

        let element = if user_workspaces.is_empty() {
            element
        } else {
            // User workspaces
            element.children(user_workspaces)
        };

        // Workspace addition button
        element.child(
            v_flex()
                .w(self.layout_state.read(cx).workspace_list_width)
                .h_full()
                .justify_center()
                .items_center()
                .child(Icon::new(IconName::Plus).size_8())
                .on_mouse_down(MouseButton::Left, {
                    let list = self.state.clone();

                    move |_event, window, cx| {
                        let path = AppPath::global(cx).clone();
                        let list = list.clone();

                        window
                            .spawn(cx, async move |cx| {
                                let data = create_workspace(
                                    &path,
                                    Uuid::new_v4(),
                                    "New workspace".to_owned(),
                                )
                                .await
                                .expect("ワークスペースのデータの作成に失敗しました。");

                                list.update_in(cx, move |list, window, cx| {
                                    let rect = list.layout_state.read(cx).view_rect(window);
                                    let workspace =
                                        WorkspaceState::new(window, cx, rect, data, Vec::new())
                                            .expect("ワークスペースの作成に失敗しました。");

                                    list.add(cx, workspace).unwrap();
                                    cx.notify();
                                })
                                .unwrap();
                            })
                            .detach();
                    }
                }),
        )
    }
}

#[derive(IntoElement)]
struct WorkspaceIcon {
    list: Entity<WorkspaceListState>,
    workspace_id: Uuid,
}

impl RenderOnce for WorkspaceIcon {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let icon = self
            .list
            .read(cx)
            .list_metadata()
            .get(&self.workspace_id)
            .unwrap()
            .icon();

        match icon {
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
