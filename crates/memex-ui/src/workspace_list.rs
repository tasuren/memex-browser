use std::collections::HashMap;

use gpui::{App, Global, SharedString, Window, div, prelude::*};
use gpui_component::{ActiveTheme, Icon, Sizable, v_flex};
use uuid::Uuid;

pub struct CurrentWorkspace {
    pub data: Option<WorkspaceData>,
}

impl Global for CurrentWorkspace {}

pub fn init(cx: &mut App) {
    cx.set_global(CurrentWorkspace { data: None });
}

pub struct WorkspaceData {
    id: Uuid,
    icon: SharedString,
}

pub struct WorkspaceList {
    workspaces: HashMap<Uuid, WorkspaceData>,
    order: Vec<Uuid>,
    selected: Option<Uuid>,
}

impl WorkspaceList {
    pub fn new() -> Self {
        let mut workspaces = HashMap::new();
        let mut order = Vec::new();

        let home_id = Uuid::new_v4();
        workspaces.insert(
            home_id,
            WorkspaceData {
                id: home_id,
                icon: "🏠".into(),
            },
        );
        order.push(home_id);

        let mut push = |f: fn(Uuid) -> WorkspaceData| {
            let id = Uuid::new_v4();
            order.push(id);
            workspaces.insert(id, f(id));
        };

        push(|id| WorkspaceData {
            id,
            icon: "🔬".into(),
        });
        push(|id| WorkspaceData {
            id,
            icon: "😳".into(),
        });

        Self {
            workspaces,
            order,
            selected: None,
        }
    }
}

impl Render for WorkspaceList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
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
                        self.selected.is_some(),
                        |this| this.bg(cx.theme().accent),
                        |this| this.bg(cx.theme().accent.alpha(0.4)),
                    )
                    .child(Icon::empty().path("icons/house.svg").large()),
            )
            .child(div().w_3_4().border_1().border_color(cx.theme().border))
            // User workspaces
            .children(self.order.iter().enumerate().map(|(ix, id)| {
                let workspace = self.workspaces.get(id).unwrap();

                v_flex()
                    .id(ix)
                    .size_12()
                    .when(
                        self.selected
                            .is_some_and(|selected| workspace.id == selected),
                        |this| this.bg(cx.theme().primary.alpha(0.2)),
                    )
                    .rounded_xl()
                    .justify_center()
                    .text_center()
                    .text_3xl()
                    .child(workspace.icon.clone())
            }))
    }
}
