use std::path::PathBuf;

use anyhow::{Result, anyhow};
use gpui::{
    App, Application, Bounds, Entity, SharedString, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, size,
};
use gpui_component::{
    ActiveTheme, Root, Theme, ThemeRegistry, TitleBar, h_flex,
    resizable::{ResizableState, h_resizable, resizable_panel},
    v_flex,
};

use crate::{
    controller::Controller,
    exproler::Exproler,
    ui_consts::{TOP_TAB_BAR_HEIGHT, WORKSPACE_LIST_WIDTH},
    workspace_list::{CurrentWorkspace, WorkspaceList},
};

mod controller;
mod exproler;
mod tab_bar;
mod ui_consts;
mod workspace_list;

struct MemexBrowser {
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
                            .when(cx.global::<CurrentWorkspace>().data.is_some(), |this| {
                                this.child(
                                    resizable_panel().size(px(200.)).child(
                                        div()
                                            .size_full()
                                            .border_r_1()
                                            .border_color(cx.theme().border)
                                            .child(self.exproler.clone()),
                                    ),
                                )
                            })
                            .child(h_flex().size_full().p_4().into_any_element()),
                    ),
            )
    }
}

pub fn init_theme(cx: &mut App) {
    let theme_name = "Ayu Dark";
    let theme_dir = PathBuf::from("./themes");
    let on_load = move |cx: &mut App| {
        if let Some(theme) = ThemeRegistry::global(cx).themes().get(theme_name).cloned() {
            Theme::global_mut(cx).apply_config(&theme);
        }
    };

    if let Err(err) = ThemeRegistry::watch_dir(theme_dir, cx, on_load) {
        eprintln!("Failed to watch themes directory: {}", err);
    }
}

fn main() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        gpui_component::init(cx);
        init_theme(cx);

        workspace_list::init(cx);

        let bounds = Bounds::centered(None, size(px(1400.), px(850.0)), cx);

        cx.spawn(async move |cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitleBar::title_bar_options()),
                    ..Default::default()
                },
                |window, cx| {
                    cx.new(|cx| {
                        Root::new(
                            cx.new(|cx| MemexBrowser::new(window, cx)).into(),
                            window,
                            cx,
                        )
                    })
                },
            )?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();

        cx.activate(true);
    });
}

#[derive(rust_embed::RustEmbed)]
#[folder = "./assets"]
#[include = "icons/**/*.svg"]
pub struct Assets;

impl gpui::AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        Self::get(path)
            .map(|f| Some(f.data))
            .ok_or_else(|| anyhow!("could not find asset at path `{path}`"))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect())
    }
}
