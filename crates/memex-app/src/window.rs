use gpui::{App, Bounds, WindowBounds, WindowOptions, prelude::*, px, size};
use memex_backend::SystemContext;
use memex_cef::UIThreadMarker;
use raw_window_handle::HasWindowHandle;

pub fn setup_window(cx: &mut App) -> anyhow::Result<()> {
    let bounds = Bounds::centered(None, size(px(1400.), px(850.0)), cx);

    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(gpui_component::TitleBar::title_bar_options()),
            ..Default::default()
        },
        |window, cx| {
            let manager = cx.global::<SystemContext>().workspace_manager().clone();
            let window_handle = window.window_handle().unwrap().as_raw();

            // TODO: spawnを使う。
            futures_lite::future::block_on(async move {
                // 開いたウィンドウでホームワークスペースを開いておく。
                let mut manager = manager.lock().unwrap();
                let home_id = manager.home();
                let utm = UIThreadMarker::new().unwrap();

                manager
                    .open(utm, home_id, window_handle)
                    .await
                    .expect("ホームワークスペースを開くのに失敗しました。");
            });

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
