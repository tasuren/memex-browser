use crate::cef_impl::render_process_handler::{BrowserCreateListener, RenderProcessHandlerService};

#[derive(Clone)]
pub struct CefContext {
    _render_process_handler: cef::RenderProcessHandler,
    browser_create_listener: BrowserCreateListener,
}

impl CefContext {
    pub fn new() -> Self {
        let (render_process_handler, browser_create_listener) =
            RenderProcessHandlerService::create();

        Self {
            _render_process_handler: render_process_handler,
            browser_create_listener,
        }
    }

    pub async fn wait_for_browser(&self) -> Option<cef::Browser> {
        self.browser_create_listener.recv().await.ok()
    }
}
