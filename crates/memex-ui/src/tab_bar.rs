use std::collections::HashMap;

use gpui::{Entity, MouseButton, SharedString, actions, prelude::*, px};
use gpui_component::*;
use uuid::Uuid;

actions!(tab_actions, [NextTab, PreviousTab]);

pub struct Tab {
    _id: Uuid,
    title: SharedString,
}

pub struct TabBarState {
    tabs: HashMap<Uuid, Tab>,
    tab_order: Vec<Uuid>,
    tab_history: Vec<Uuid>,
    selected: Uuid,
}

impl TabBarState {
    fn select(&mut self, id: Uuid) {
        self.selected = id;

        if let Some(previous_pos) = self
            .tab_history
            .iter()
            .position(|candidate| *candidate == id)
        {
            self.tab_history.remove(previous_pos);
        }

        self.tab_history.insert(0, id)
    }
}

pub struct TabBar {
    state: Entity<TabBarState>,
}

impl TabBar {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let mut tabs = HashMap::new();
        let mut tab_order = Vec::new();
        let mut tab_history = Vec::new();

        let id = Uuid::new_v4();
        let tab = Tab {
            _id: id,
            title: "テストタブ1".into(),
        };
        tab_order.push(id);
        tabs.insert(id, tab);
        tab_history.push(id);
        let selected = id;

        let id = Uuid::new_v4();
        let tab = Tab {
            _id: id,
            title: "テストタブ2".into(),
        };
        tab_order.push(id);
        tabs.insert(id, tab);

        Self {
            state: cx.new(|_| TabBarState {
                tabs,
                tab_order,
                tab_history,
                selected,
            }),
        }
    }
}

impl Render for TabBar {
    fn render(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);

        h_flex()
            .id("tab-bar")
            .size_full()
            .justify_start()
            .items_center()
            .gap_1()
            .pt(px(6.))
            .px_2()
            .children(
                self.state
                    .read(cx)
                    .tab_order
                    .iter()
                    .cloned()
                    .enumerate()
                    .map(|(ix, id)| {
                        let tab = state.tabs.get(&id).unwrap();

                        h_flex()
                            .id(ix)
                            .justify_between()
                            .items_center()
                            .w_48()
                            .h_full()
                            .rounded_t_xl()
                            .px_4()
                            .when_else(
                                self.state.read(cx).selected == id,
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
                            .child(
                                h_flex()
                                    .justify_start()
                                    .items_center()
                                    .child(tab.title.clone()),
                            )
                            .child(Icon::empty().path("icons/x.svg"))
                            .on_mouse_up(MouseButton::Left, {
                                let state = self.state.clone();

                                move |_event, _window, cx| {
                                    state.update(cx, |state, _| {
                                        if state.selected != id {
                                            state.select(id)
                                        }
                                    })
                                }
                            })
                    }),
            )
    }
}
