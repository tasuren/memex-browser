use cef::*;

use crate::define_cef_service;

define_cef_service! {
    #[derive_cef(WrapRequestContextHandler)]
    pub struct RequestContextHandlerService {
        sys: *mut cef::rc::RcImpl<sys::cef_request_context_handler_t, Self>,
    }
}

impl RequestContextHandlerService {
    pub fn create() -> RequestContextHandler {
        RequestContextHandler::new(Self {
            sys: Default::default(),
        })
    }
}

impl ImplRequestContextHandler for RequestContextHandlerService {
    fn get_raw(&self) -> *mut sys::_cef_request_context_handler_t {
        self.sys.cast()
    }
}
