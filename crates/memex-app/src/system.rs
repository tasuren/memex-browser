use futures_lite::future;
use gpui::{App, Application};
use memex_backend::{SystemContext, data::AppPath};

use crate::setup_window;

pub fn boot() {
    init_log();

    let app = Application::new();

    let system = future::block_on(async {
        let path = AppPath::new(crate::APP_IDENTIFIER.to_owned())
            .await
            .expect("システムのセーブデータ格納場所の構築に失敗しました。");

        SystemContext::new(app.background_executor(), app.foreground_executor(), path)
            .await
            .expect("システムの初期化に失敗しました。")
    });

    app.with_assets(crate::foundation::Assets)
        .run(move |cx: &mut App| {
            init_cef(cx, system);
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
pub fn init_cef(cx: &mut App, system: SystemContext) {
    let mut event_loop = match memex_cef::boot().expect("CEFの起動処理に失敗しました。")
    {
        Some(event_loop) => event_loop,
        None => std::process::exit(0),
    };
    cx.set_global(system);

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
