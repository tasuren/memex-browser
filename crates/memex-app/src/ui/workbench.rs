use gpui::{App, Entity, Window, div, prelude::*};
use gpui_component::{
    ActiveTheme, h_flex,
    resizable::{ResizableState, h_resizable, resizable_panel},
    v_flex,
};
use memex_backend::LayoutState;

use crate::ui::{Exproler, TitleBar, WorkspaceList};

/// ワークスペースを開いている前提のView。
/// ワークスペース一覧を読み込み終わった後にしか作れない。
pub struct Workbench {
    layout_state: Entity<LayoutState>,

    workspace_list: Entity<WorkspaceList>,
    title_bar: Entity<TitleBar>,
    exproler: Entity<Exproler>,

    workspace_box_state: Entity<ResizableState>,
}

impl Workbench {
    pub fn new(
        cx: &mut App,
        layout_state: Entity<LayoutState>,
        workspace_list: Entity<WorkspaceList>,
        title_bar: Entity<TitleBar>,
    ) -> Entity<Self> {
        cx.new(move |cx| Self {
            layout_state,
            workspace_list,
            title_bar,
            exproler: Exproler::new(cx),

            workspace_box_state: ResizableState::new(cx),
        })
    }
}

impl Render for Workbench {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        let layout_state = self.layout_state.read(cx);

        h_flex()
            .id("window-body")
            .size_full()
            .child(
                v_flex()
                    .id("workspace-list")
                    .w(layout_state.workspace_list_width)
                    .h_full()
                    .mt(layout_state.top_tab_bar_height)
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
                            .child(self.title_bar.clone()),
                    )
                    .child(
                        h_resizable("workspace", self.workspace_box_state.clone())
                            .when(
                                {
                                    let list_state = self.workspace_list.read(cx).state().read(cx);
                                    list_state.selected() != list_state.home()
                                },
                                |this| {
                                    this.child(
                                        resizable_panel().size(layout_state.exproler_width).child(
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
