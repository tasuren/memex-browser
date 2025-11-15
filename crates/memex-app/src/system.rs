use futures_lite::future::block_on;
use gpui::{App, Application};
use memex_backend::init_global_state;

use crate::{APP_IDENTIFIER, setup_window};

pub fn boot() {
    init_log();

    Application::new()
        .with_assets(crate::foundation::Assets)
        .run(move |cx: &mut App| {
            block_on(init_global_state(cx, APP_IDENTIFIER.to_owned())).unwrap();

            init_cef(cx);
            init_ui(cx);

            setup_window(cx).expect("ウィンドウの作成に失敗しました。");
            cx.activate(true);
        });
}

#[inline]
pub fn init_log() {
    env_logger::init();
}

#[inline]
pub fn init_cef(cx: &mut App) {
    let mut event_loop = match memex_cef::boot().expect("CEFの起動処理に失敗しました。")
    {
        Some(event_loop) => event_loop,
        None => std::process::exit(0),
    };

    cx.spawn(async move |_| {
        event_loop.start(|task| task()).await;
    })
    .detach();

    cx.on_app_quit(|_cx| async { memex_cef::teardown() })
        .detach();
}

#[inline]
pub fn init_ui(cx: &mut App) {
    #[cfg(target_os = "macos")]
    crate::platform_impl::mac::application::impl_cef_protocol_for_gpui_app();

    // UIに使う周辺の初期化。
    gpui_component::init(cx);
    crate::foundation::init_theme(cx);
}
