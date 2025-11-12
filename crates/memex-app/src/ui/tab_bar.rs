use gpui::{AsyncApp, MouseButton, actions, prelude::*, px};
use gpui_component::*;
use memex_backend::SystemContext;
use memex_cef::UIThreadMarker;
use raw_window_handle::HasWindowHandle;

actions!(tab_actions, [NextTab, PreviousTab]);

pub struct TabBar {}

impl TabBar {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for TabBar {
    fn render(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        let system = cx.global::<SystemContext>();
        let manager = system.workspace_manager().lock().unwrap();
        let workspace = manager.get(manager.selected()).unwrap();

        h_flex()
            .id("tab-bar")
            .size_full()
            .justify_start()
            .items_center()
            .gap_1()
            .pt(px(6.))
            .px_2()
            .children(
                workspace
                    .tab_order()
                    .iter()
                    .cloned()
                    .enumerate()
                    .map(|(ix, id)| {
                        let system = cx.global::<SystemContext>();
                        let manager = system.workspace_manager().lock().unwrap();
                        let workspace = manager.get(manager.selected()).unwrap();
                        let tab = workspace.get_tab(id).unwrap();

                        h_flex()
                            .id(ix)
                            .justify_between()
                            .items_center()
                            .w_48()
                            .h_full()
                            .rounded_t_xl()
                            .px_4()
                            .when_else(
                                workspace.selected_tab() == Some(id),
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
                            .on_mouse_up(MouseButton::Left, move |_event, _window, cx| {
                                let system = cx.global::<SystemContext>();
                                let mut manager = system.workspace_manager().lock().unwrap();
                                let workspace_id = manager.selected();
                                let workspace = manager.get_mut(workspace_id).unwrap();
                                let utm = UIThreadMarker::new().unwrap();

                                workspace.select(utm, id).unwrap();
                            })
                    }),
            )
            .child(
                v_flex()
                    .justify_center()
                    .items_center()
                    .child(Icon::new(IconName::Plus).size_10())
                    .on_mouse_up(MouseButton::Left, |_event, window, cx| {
                        println!("aa");
                        let system = cx.global::<SystemContext>();
                        let manager = system.workspace_manager().clone();
                        let window_handle = window.window_handle().unwrap().as_raw();

                        cx.spawn(move |_cx: &mut AsyncApp| async move {
                            let mut manager = manager.lock().unwrap();
                            let workspace_id = manager.selected();
                            let workspace = manager.get_mut(workspace_id).unwrap();

                            workspace
                                .create_tab(window_handle)
                                .await
                                .expect("新しいタブを開くのに失敗しました。");
                        })
                        .detach();
                    }),
            )
    }
}
