use std::rc::Rc;

use anyhow::Context;
use cef::{BrowserSettings, RequestContext};

use crate::cef_impl::RequestContextHandlerService;

pub type SharedBrowserSettings = Rc<BrowserSettings>;

#[derive(Clone)]
pub struct Profile {
    pub browser_settings: SharedBrowserSettings,
    pub request_context: RequestContext,
}

impl Profile {
    pub fn new() -> anyhow::Result<Self> {
        let mut request_context_handler_service = RequestContextHandlerService::create();

        let request_context = cef::request_context_create_context(
            Some(&cef::RequestContextSettings {
                accept_language_list: "ja,en-US".into(),
                ..Default::default()
            }),
            Some(&mut request_context_handler_service),
        )
        .context("Failed to prepare request context")?;

        Ok(Self {
            browser_settings: Rc::new(BrowserSettings::default()),
            request_context,
        })
    }
}
