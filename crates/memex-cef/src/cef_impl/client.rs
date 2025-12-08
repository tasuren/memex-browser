use cef::*;

use crate::{cef_impl::LifeSpanHandlerService, helper::define_cef_service};

define_cef_service! {
    #[derive_cef(WrapClient)]
    pub struct ClientService {
        sys: *mut cef::rc::RcImpl<sys::cef_client_t, Self>,
        life_span_handler: LifeSpanHandler,
    }
}

impl ClientService {
    pub fn create() -> Client {
        Client::new(Self {
            sys: Default::default(),
            life_span_handler: LifeSpanHandlerService::create(),
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
