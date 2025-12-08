use cef::*;

use crate::{
    cef_impl::BrowserProcessHandlerService, event_loop::PumpTx, helper::define_cef_service,
};

define_cef_service! {
    #[derive_cef(cef::WrapApp)]
    pub struct AppService {
        sys: *mut cef::rc::RcImpl<sys::cef_app_t, Self>,
        browser_process_handler: BrowserProcessHandler,
    }
}

impl AppService {
    pub fn create(pump_tx: PumpTx) -> App {
        App::new(Self {
            sys: Default::default(),
            browser_process_handler: BrowserProcessHandlerService::create(pump_tx),
        })
    }
}

impl ImplApp for AppService {
    fn get_raw(&self) -> *mut sys::cef_app_t {
        self.sys.cast()
    }

    fn on_before_command_line_processing(
        &self,
        _process_type: Option<&cef::CefString>,
        command_line: Option<&mut cef::CommandLine>,
    ) {
        if let Some(command_line) = command_line {
            // キーチェーンアクセスのためのパスワード入力を起動の度に求められぬよう、
            // デバッグ時はモックを使う。
            #[cfg(debug_assertions)]
            command_line.append_switch(Some(&"use-mock-keychain".into()));
        }
    }

    fn browser_process_handler(&self) -> Option<cef::BrowserProcessHandler> {
        Some(self.browser_process_handler.clone())
    }
}
