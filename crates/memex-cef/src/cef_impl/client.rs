use cef::*;

use crate::{
    WebViewContext,
    cef_impl::{DisplayHandlerService, LifeSpanHandlerService},
    helper::define_cef_service,
};

define_cef_service! {
    #[derive_cef(WrapClient)]
    pub struct ClientService {
        sys: *mut cef::rc::RcImpl<sys::cef_client_t, Self>,
        context: WebViewContext,
        life_span_handler: LifeSpanHandler,
        display_handler: DisplayHandler,
    }
}

impl ClientService {
    pub fn create(context: WebViewContext) -> Client {
        Client::new(Self {
            sys: Default::default(),
            context: context.clone(),
            life_span_handler: LifeSpanHandlerService::create(),
            display_handler: DisplayHandlerService::create(context),
        })
    }
}

impl ImplClient for ClientService {
    fn get_raw(&self) -> *mut sys::cef_client_t {
        self.sys.cast()
    }

    fn life_span_handler(&self) -> Option<LifeSpanHandler> {
        Some(self.life_span_handler.clone())
    }
}
