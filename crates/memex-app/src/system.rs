use gpui::{App, Application};
use memex_backend::{SystemContext, data::AppPath};

use crate::setup_window;

pub fn boot() {
    let app = Application::new();

    let path = futures_lite::future::block_on(AppPath::new(crate::APP_IDENTIFIER.to_owned()))
        .expect("システムのセーブデータ格納場所の構築に失敗しました。");
    let system = SystemContext::new(app.background_executor(), app.foreground_executor(), path);

    app.with_assets(crate::foundation::Assets)
        .run(move |cx: &mut App| {
            let mut event_loop = match memex_cef::boot().expect("CEFの起動処理に失敗しました。")
            {
                Some(event_loop) => event_loop,
                None => std::process::exit(0),
            };

            cx.on_app_quit(|_cx| async { on_app_quit() }).detach();

            cx.set_global(system);
            init_ui(cx);

            setup_window(cx).expect("ウィンドウの作成に失敗しました。");
            cx.spawn(async move |_| {
                event_loop.start(|task| task()).await;
            })
            .detach();

            cx.activate(true);
        });
}

pub fn init_ui(cx: &mut App) {
    #[cfg(target_os = "macos")]
    crate::platform_impl::mac::application::impl_cef_protocol_for_gpui_app();

    // UIに使う周辺の初期化。
    gpui_component::init(cx);
    crate::foundation::init_theme(cx);

    // アプリが直接使うUIの初期化。
    crate::ui::init_workspace_list(cx);
}

pub fn on_app_quit() {
    memex_cef::teardown();
}
