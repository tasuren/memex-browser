use std::rc::Rc;

use cef::*;

use crate::{BrowserEventHandler, define_cef_service};

define_cef_service! {
    #[derive_cef(WrapDisplayHandler)]
    pub struct DisplayHandlerService {
        sys: *mut cef::rc::RcImpl<sys::cef_display_handler_t, Self>,
        event_handler: Rc<dyn BrowserEventHandler>,
    }
}

impl DisplayHandlerService {
    pub fn create(event_handler: Rc<dyn BrowserEventHandler>) -> DisplayHandler {
        DisplayHandler::new(Self {
            sys: Default::default(),
            event_handler,
        })
    }
}

impl ImplDisplayHandler for DisplayHandlerService {
    fn get_raw(&self) -> *mut sys::_cef_display_handler_t {
        self.sys.cast()
    }

    fn on_title_change(&self, _browser: Option<&mut Browser>, title: Option<&CefString>) {
        if let Some(title) = title {
            self.event_handler.on_title_change(title.to_string());
        }
    }
}
