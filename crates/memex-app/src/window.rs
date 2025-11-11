use gpui::{App, Bounds, WindowBounds, WindowOptions, prelude::*, px, size};

pub fn setup_window(cx: &mut App) -> anyhow::Result<()> {
    let bounds = Bounds::centered(None, size(px(1400.), px(850.0)), cx);

    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(gpui_component::TitleBar::title_bar_options()),
            ..Default::default()
        },
        |window, cx| {
            cx.new(|cx| {
                gpui_component::Root::new(
                    cx.new(|cx| crate::ui::MemexBrowser::new(window, cx)).into(),
                    window,
                    cx,
                )
            })
        },
    )?;

    Ok(())
}
