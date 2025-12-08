use crate::cef_impl::{BrowserCreateListener, RenderProcessHandlerService};

#[derive(Clone)]
pub struct CefContext {
    _render_process_handler: cef::RenderProcessHandler,
    browser_create_listener: BrowserCreateListener,
}

impl Default for CefContext {
    fn default() -> Self {
        let (render_process_handler, browser_create_listener) =
            RenderProcessHandlerService::create();

        Self {
            _render_process_handler: render_process_handler,
            browser_create_listener,
        }
    }
}

impl CefContext {
    pub async fn wait_for_browser(&self) -> Option<cef::Browser> {
        self.browser_create_listener.recv().await.ok()
    }
}
