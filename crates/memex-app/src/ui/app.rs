use gpui::{Entity, Window, div, prelude::*, px};
use gpui_component::{
    ActiveTheme, h_flex,
    resizable::{ResizableState, h_resizable, resizable_panel},
    v_flex,
};
use memex_backend::SystemContext;

use crate::ui::{Controller, Exproler, TOP_TAB_BAR_HEIGHT, WORKSPACE_LIST_WIDTH, WorkspaceList};

pub struct MemexBrowser {
    workspace_list: Entity<WorkspaceList>,
    controller: Entity<Controller>,
    exproler: Entity<Exproler>,

    workspace_box_state: Entity<ResizableState>,
}

impl MemexBrowser {
    pub fn new(window: &mut Window, cx: &mut Context<'_, Self>) -> Self {
        Self {
            workspace_list: cx.new(|_cx| WorkspaceList::new()),
            controller: cx.new(|cx| Controller::new(window, cx)),
            exproler: cx.new(|cx| Exproler::new(cx)),

            workspace_box_state: ResizableState::new(cx),
        }
    }
}

impl Render for MemexBrowser {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let system = cx.global::<SystemContext>();
        let workspace_manager = system.workspace_manager().lock().unwrap();

        h_flex()
            .id("window-body")
            .size_full()
            .child(
                v_flex()
                    .id("workspace-list")
                    .w(WORKSPACE_LIST_WIDTH)
                    .h_full()
                    .mt(TOP_TAB_BAR_HEIGHT)
                    .pt_4()
                    .px_2()
                    .items_center()
                    .child(self.workspace_list.clone()),
            )
            .child(
                v_flex()
                    .size_full()
                    .border_l_1()
                    .border_color(cx.theme().border)
                    .child(
                        v_flex()
                            .id("controller")
                            .w_full()
                            .border_b_1()
                            .border_color(cx.theme().border)
                            .child(self.controller.clone()),
                    )
                    .child(
                        h_resizable("workspace", self.workspace_box_state.clone())
                            .when(
                                workspace_manager.selected() != workspace_manager.home(),
                                |this| {
                                    this.child(
                                        resizable_panel().size(px(200.)).child(
                                            div()
                                                .size_full()
                                                .border_r_1()
                                                .border_color(cx.theme().border)
                                                .child(self.exproler.clone()),
                                        ),
                                    )
                                },
                            )
                            .child(h_flex().size_full().p_4().into_any_element()),
                    ),
            )
    }
}
