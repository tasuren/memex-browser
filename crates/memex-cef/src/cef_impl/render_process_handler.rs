use cef::*;

use crate::helper::define_cef_service;

pub type BrowserCreateNotifier = async_channel::Sender<Browser>;
pub type BrowserCreateListener = async_channel::Receiver<Browser>;

define_cef_service! {
    #[derive_cef(WrapRenderProcessHandler)]
    pub struct RenderProcessHandlerService {
        sys: *mut cef::rc::RcImpl<sys::cef_render_process_handler_t, Self>,
        notifier: BrowserCreateNotifier,
    }
}

impl RenderProcessHandlerService {
    pub fn create() -> (RenderProcessHandler, BrowserCreateListener) {
        let (notifier, listener) = async_channel::unbounded();

        (
            RenderProcessHandler::new(Self {
                sys: Default::default(),
                notifier,
            }),
            listener,
        )
    }
}

impl ImplRenderProcessHandler for RenderProcessHandlerService {
    fn get_raw(&self) -> *mut sys::_cef_render_process_handler_t {
        self.sys.cast()
    }

    fn on_browser_created(
        &self,
        browser: Option<&mut Browser>,
        _extra_info: Option<&mut DictionaryValue>,
    ) {
        if let Some(browser) = browser.cloned() {
            let _ = self.notifier.send_blocking(browser);
        } else {
            log::warn!("Some browser is created but I received `None`.");
        }
    }
}
