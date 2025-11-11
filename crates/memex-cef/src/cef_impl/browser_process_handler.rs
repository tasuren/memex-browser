use cef::*;

use crate::{
    event_loop::{PumpDelayMs, PumpTx},
    helper::define_cef_service,
};

define_cef_service! {
    #[derive_cef(cef::WrapBrowserProcessHandler)]
    pub struct BrowserProcessHandlerService {
        sys: *mut cef::rc::RcImpl<sys::cef_browser_process_handler_t, Self>,
        pump_tx: PumpTx,
    }
}

impl BrowserProcessHandlerService {
    pub fn create(pump_tx: PumpTx) -> BrowserProcessHandler {
        BrowserProcessHandler::new(Self {
            sys: Default::default(),
            pump_tx,
        })
    }
}

impl cef::ImplBrowserProcessHandler for BrowserProcessHandlerService {
    fn get_raw(&self) -> *mut sys::cef_browser_process_handler_t {
        self.sys.cast()
    }

    fn on_schedule_message_pump_work(&self, delay_ms: i64) {
        let _ = self.pump_tx.send(PumpDelayMs(delay_ms as _));
    }
}
