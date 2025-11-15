use gpui::{App, Entity, MouseButton, Window, actions, prelude::*, px};
use gpui_component::*;
use memex_backend::{LayoutState, WorkspaceState};

actions!(tab_actions, [NextTab, PreviousTab]);

pub struct TabBar {
    layout_state: Entity<LayoutState>,
    workspace: Entity<WorkspaceState>,
}

impl TabBar {
    pub fn new(
        cx: &mut App,
        layout_state: Entity<LayoutState>,
        workspace: Entity<WorkspaceState>,
    ) -> Entity<Self> {
        cx.new(|_| Self {
            layout_state,
            workspace,
        })
    }
}

impl Render for TabBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .id("tab-bar")
            .size_full()
            .justify_start()
            .items_center()
            .gap_1()
            .pt(px(6.))
            .px_2()
            .children(
                self.workspace
                    .read(cx)
                    .tab_order()
                    .iter()
                    .cloned()
                    .enumerate()
                    .map(|(ix, id)| {
                        let tab = self.workspace.read(cx).get_tab(id).unwrap().read(cx);

                        h_flex()
                            .id(ix)
                            .justify_between()
                            .items_center()
                            .w_48()
                            .h_full()
                            .rounded_t_xl()
                            .px_4()
                            .when_else(
                                self.workspace.read(cx).selected_tab() == Some(id),
                                |this| this.bg(cx.theme().background),
                                |this| {
                                    this.bg(cx.theme().title_bar).hover(|style| {
                                        style
                                            .h_5_6()
                                            .rounded_md()
                                            .bg(cx.theme().foreground.alpha(0.3))
                                    })
                                },
                            )
                            .child(h_flex().justify_start().items_center().child(tab.title()))
                            .child(Icon::empty().path("icons/x.svg"))
                            .on_mouse_up(MouseButton::Left, {
                                let workspace = self.workspace.clone();

                                move |_event, _window, cx| {
                                    cx.update_entity(&workspace, |workspace, cx| {
                                        workspace.select(cx, id);
                                    });
                                }
                            })
                    }),
            )
            .child(
                v_flex()
                    .size(self.layout_state.read(cx).top_tab_bar_height)
                    .justify_center()
                    .items_center()
                    .child(Icon::new(IconName::Plus).size_6())
                    .on_mouse_up(MouseButton::Left, {
                        let workspace = self.workspace.clone();

                        move |_event, window, cx| {
                            workspace.update(cx, |workspace, cx| {
                                workspace
                                    .create_tab(window, cx)
                                    .expect("新しいタブを開くのに失敗しました。");
                            });
                        }
                    }),
            )
    }
}
