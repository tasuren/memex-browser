use cef::*;

use crate::helper::define_cef_service;

define_cef_service! {
    #[derive_cef(cef::WrapLifeSpanHandler)]
    pub struct LifeSpanHandlerService {
        sys: *mut cef::rc::RcImpl<sys::cef_life_span_handler_t, Self>,
    }
}

impl LifeSpanHandlerService {
    pub fn create() -> LifeSpanHandler {
        LifeSpanHandler::new(Self {
            sys: Default::default(),
        })
    }
}

impl ImplLifeSpanHandler for LifeSpanHandlerService {
    fn get_raw(&self) -> *mut sys::cef_life_span_handler_t {
        self.sys.cast()
    }

    fn do_close(&self, browser: Option<&mut cef::Browser>) -> ::std::os::raw::c_int {
        // remove browser from window
        if let Some(_browser) = browser.and_then(|b| b.host()) {
            log::info!("todo: close")
        } else {
            log::warn!("called `do_close` but Browser/BrowserHost is not available");
        }

        // prevent CEF from closing window
        true as _
    }
}
