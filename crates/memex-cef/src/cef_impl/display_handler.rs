use cef::*;

use crate::{WebViewContext, define_cef_service};

define_cef_service! {
    #[derive_cef(WrapDisplayHandler)]
    pub struct DisplayHandlerService {
        sys: *mut cef::rc::RcImpl<sys::cef_display_handler_t, Self>,
        context: WebViewContext,
    }
}

impl DisplayHandlerService {
    pub fn create(context: WebViewContext) -> DisplayHandler {
        DisplayHandler::new(Self {
            sys: Default::default(),
            context,
        })
    }
}

impl ImplDisplayHandler for DisplayHandlerService {
    fn get_raw(&self) -> *mut sys::_cef_display_handler_t {
        self.sys.cast()
    }

    fn on_title_change(&self, _browser: Option<&mut Browser>, title: Option<&CefString>) {
        if let Some(title) = title {
            self.context
                .event_handler()
                .on_title_change(title.to_string());
        }
    }
}
