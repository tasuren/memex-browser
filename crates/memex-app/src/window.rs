use gpui::{App, Bounds, WindowBounds, WindowOptions, prelude::*, px, size};

pub fn setup_window(cx: &mut App) -> anyhow::Result<()> {
    let bounds = Bounds::centered(None, size(px(1300.), px(800.0)), cx);

    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(gpui_component::TitleBar::title_bar_options()),
            ..Default::default()
        },
        |window, cx| {
            let browser_window = crate::ui::BrowserWindow::new(window, cx);

            cx.new(|cx| gpui_component::Root::new(browser_window.into(), window, cx))
        },
    )?;

    Ok(())
}
