use std::{path::Path, rc::Rc};

use anyhow::Context as _;

use crate::cef_impl::RequestContextHandlerService;

pub type SharedBrowserSettings = Rc<cef::BrowserSettings>;

#[derive(Clone)]
pub struct Profile {
    pub browser_settings: SharedBrowserSettings,
    pub request_context: cef::RequestContext,
}

impl Profile {
    pub fn new(cache_path: &Path) -> anyhow::Result<Self> {
        let mut request_context_handler_service = RequestContextHandlerService::create();

        let browser_settings = cef::BrowserSettings::default();
        let request_context_settings = cef::RequestContextSettings {
            accept_language_list: "ja,en-US".into(),
            cache_path: cache_path
                .to_str()
                .context("`cache_path`の文字列化に失敗")?
                .into(),
            ..Default::default()
        };

        let request_context = cef::request_context_create_context(
            Some(&request_context_settings),
            Some(&mut request_context_handler_service),
        )
        .context("リクエストコンテキストの作成に失敗")?;

        Ok(Self {
            browser_settings: Rc::new(browser_settings),
            request_context,
        })
    }
}
