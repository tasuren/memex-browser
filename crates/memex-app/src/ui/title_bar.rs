use gpui::{App, Entity, div, prelude::*, px};
use gpui_component::{
    ActiveTheme, Icon, IconName, Sizable,
    button::{Button, ButtonVariants},
    h_flex,
    input::{InputState, TextInput},
    v_flex,
};
use memex_backend::{LayoutState, WorkspaceListState, WorkspaceState};

use crate::ui::{
    consts::{TOP_TAB_BAR_HEIGHT, URL_BAR_HEIGHT},
    tab_bar::TabBar,
};

pub struct TitleBar {
    tabs: Entity<TabBar>,
    url: Entity<InputState>,
}

impl TitleBar {
    pub fn new(
        window: &mut gpui::Window,
        cx: &mut App,
        layout_state: Entity<LayoutState>,
        workspace_list_state: Entity<WorkspaceListState>,
        workspace_state: Entity<WorkspaceState>,
    ) -> Entity<Self> {
        cx.new(|cx: &mut Context<'_, _>| {
            // ワークスペースが再読み込みされたら、タブバーも作り直す。
            cx.observe(&workspace_list_state, {
                let layout_state = layout_state.clone();

                move |title_bar: &mut TitleBar, entity, cx: &mut Context<'_, _>| {
                    let workspace = entity.read(cx).current().clone();
                    title_bar.tabs = TabBar::new(cx, layout_state.clone(), workspace);
                }
            })
            .detach();

            Self {
                tabs: TabBar::new(cx, layout_state, workspace_state),
                url: cx
                    .new(|cx| InputState::new(window, cx).default_value("https://www.google.com/")),
            }
        })
    }
}

impl Render for TitleBar {
    fn render(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .child(
                div()
                    .w_full()
                    .h(TOP_TAB_BAR_HEIGHT)
                    .bg(cx.theme().title_bar)
                    .child(self.tabs.clone()),
            )
            .child(
                h_flex()
                    .h(URL_BAR_HEIGHT)
                    .gap_2()
                    .items_center()
                    .text_color(cx.theme().foreground)
                    .child(
                        h_flex()
                            .gap_1()
                            .child(
                                Button::new("back")
                                    .icon(Icon::new(IconName::ChevronLeft))
                                    .ghost()
                                    .size_10()
                                    .with_size(px(28.)),
                            )
                            .child(
                                Button::new("forward")
                                    .icon(Icon::new(IconName::ChevronRight))
                                    .ghost()
                                    .size_10()
                                    .with_size(px(28.)),
                            )
                            .child(
                                Button::new("reload")
                                    .icon(Icon::empty().path("icons/rotate-cw.svg"))
                                    .ghost()
                                    .size_10()
                                    .with_size(px(28.)),
                            ),
                    )
                    .child(TextInput::new(&self.url))
                    .child(
                        Button::new("menu")
                            .icon(Icon::new(IconName::EllipsisVertical))
                            .ghost()
                            .size_10()
                            .with_size(px(28.)),
                    ),
            )
    }
}
