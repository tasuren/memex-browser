use std::ffi::c_void;

use anyhow::Context;
use cef::{CefStringUtf16, ImplBrowser, ImplBrowserHost, ImplFrame};
use raw_window_handle::RawWindowHandle;

use crate::{
    UIThreadMarker,
    profile::Profile,
    utils::{self, Rect},
};

#[derive(Clone)]
pub struct Browser {
    sys: cef::Browser,
}

impl Browser {
    pub fn new(
        profile: &mut Profile,
        parent_window: RawWindowHandle,
        initial_url: &str,
        rect: utils::Rect,
    ) -> anyhow::Result<Self> {
        let view = match parent_window {
            RawWindowHandle::AppKit(handle) => handle.ns_view.as_ptr(),
            _ => unimplemented!(),
        };

        let window_info = cef::WindowInfo {
            #[cfg(target_os = "macos")]
            parent_view: view,
            #[cfg(target_os = "windows")]
            parent_window: view,
            bounds: rect.into(),
            ..Default::default()
        };

        let browser = cef::browser_host_create_browser_sync(
            Some(&window_info),
            Some(&mut profile.client),
            Some(&initial_url.into()),
            Some(&profile.browser_settings),
            None,
            Some(&mut profile.request_context),
        );
        // anyhow::ensure!(result == 1, "Failed to create browser.");

        // let browser = manager
        //     .wait_for_browser()
        //     .await
        //     .context("Failed to retrieve browser from CEF.")?;

        Ok(Self {
            sys: browser.context("ブラウザの作成に失敗しました。")?,
        })
    }

    pub fn on_resize(&self, _rect: Rect) {
        log::info!("TODO: on_resize");
    }

    pub fn view_handle(&self, utm: UIThreadMarker) -> Option<*mut c_void> {
        let _ = utm;

        self.sys
            .host()
            .map(|browser_host| browser_host.window_handle())
    }

    pub fn current_url(&self) -> String {
        CefStringUtf16::from(
            &self
                .sys
                .main_frame()
                .expect("Failed to get main frame.")
                .url(),
        )
        .to_string()
    }

    pub fn go_back(&self) {
        log::debug!("go_back");
        self.sys.go_back();
    }

    pub fn go_forward(&self) {
        log::debug!("go_forward");
        self.sys.go_forward();
    }

    pub fn can_go_back(&self) -> bool {
        self.sys.can_go_back() == 1
    }

    pub fn can_go_forward(&self) -> bool {
        self.sys.can_go_forward() == 1
    }

    pub fn reload(&self) {
        log::debug!("reload");
        self.sys.reload();
    }

    pub fn hard_reload(&self) {
        log::debug!("hard reload");
        self.sys.reload_ignore_cache();
    }

    pub fn close(&self, utm: UIThreadMarker) -> anyhow::Result<()> {
        log::debug!("close");
        let _ = utm;

        self.sys
            .host()
            .context("The browser host is not available yet.")?
            .close_browser(0);

        Ok(())
    }

    pub fn title(&self) -> anyhow::Result<String> {
        Ok("title TODO".to_owned())
    }
}
