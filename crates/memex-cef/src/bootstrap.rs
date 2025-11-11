use cef::{args::Args, *};

use crate::{cef_impl::app::AppService, event_loop::EventLoopHandle};

/// CEFをセットアップする。最初に呼ばれるべき。
/// ブラウザプロセスとしての起動であれば、ブラウザのイベントループの操作用ハンドルを返す。
/// そうでなければ`None`を返す。`None`の場合、それ以上やることはないので終了すべき。
pub fn boot() -> anyhow::Result<Option<EventLoopHandle>> {
    #[cfg(target_os = "macos")]
    let _loader = {
        let loader = library_loader::LibraryLoader::new(&std::env::current_exe().unwrap(), false);
        assert!(loader.load());
        loader
    };

    let _ = api_hash(sys::CEF_API_VERSION_LAST, 0);

    let args = Args::new();
    let cmd = args.as_cmd_line().unwrap();

    let switch = CefString::from("type");
    let is_browser_process = cmd.has_switch(Some(&switch)) != 1;

    let (event_loop, pump_tx) = EventLoopHandle::new();
    let mut app = AppService::create(pump_tx);

    let ret = execute_process(
        Some(args.as_main_args()),
        Some(&mut app),
        std::ptr::null_mut(),
    );

    if is_browser_process {
        anyhow::ensure!(ret == -1, "ブラウザプロセスの起動に失敗しました。");
        log::info!("ブラウザプロセスを起動。");
    } else {
        let process_type = CefString::from(&cmd.switch_value(Some(&switch)));
        log::info!("プロセス{process_type}を起動。");
        anyhow::ensure!(ret >= 0, "プロセス{process_type}の起動に失敗しました。");

        return Ok(None);
    }

    // TODO: 設定をプロファイル毎に分けるべきなのかを確認する。
    let settings = Settings {
        ..Default::default()
    };
    assert_eq!(
        initialize(
            Some(args.as_main_args()),
            Some(&settings),
            Some(&mut app),
            std::ptr::null_mut(),
        ),
        1
    );

    Ok(Some(event_loop))
}

/// CEFを終了する。
pub fn teardown() {
    shutdown();
}
