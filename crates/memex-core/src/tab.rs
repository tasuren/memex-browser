use memex_cef::{EventHandler, WebView, WebViewContext};

use crate::{
    BrowserContext, Id, TabMarker,
    db::{TabData, TabLocationData, update_location},
};

pub struct Tab {
    id: Id<TabMarker>,
    browser_context: BrowserContext,
    pub(crate) initial_location: TabLocationData,
    webview: Option<WebView>,
    webview_context: WebViewContext,
}

impl Tab {
    pub fn new(browser_context: BrowserContext, data: TabData) -> anyhow::Result<Self> {
        let event_handler = TabEventHandler {
            id: data.id,
            context: browser_context.clone(),
        };
        let webview_context = WebViewContext::new(event_handler);

        Ok(Self {
            id: data.id,
            browser_context,
            initial_location: data.location,
            webview: None,
            webview_context,
        })
    }

    pub fn id(&self) -> Id<TabMarker> {
        self.id
    }

    pub fn is_loaded(&self) -> bool {
        self.webview.is_some()
    }

    pub fn load(&mut self) -> anyhow::Result<()> {
        anyhow::ensure!(self.webview.is_none(), "既にこのタブはロード済みです。");
        let url = get_url(&self.initial_location);

        self.webview = Some(WebView::new(
            &mut self.browser_context.profile,
            self.webview_context.clone(),
            self.browser_context.window_handle,
            url,
            self.browser_context.rect.get(),
        )?);

        Ok(())
    }

    pub fn location(&self) -> TabLocationData {
        if let Some(webview) = self.webview.as_ref() {
            TabLocationData::WebPage {
                url: webview.current_url(),
            }
        } else {
            self.initial_location.clone()
        }
    }

    pub async fn navigate(&mut self, location: TabLocationData) -> anyhow::Result<()> {
        update_location(&self.browser_context.db, self.id, &location).await;

        if let Some(webview) = self.webview.as_ref() {
            webview.navigate(get_url(&location))?;
        } else {
            self.initial_location = location;
        }

        Ok(())
    }
}

fn get_url(location: &TabLocationData) -> &str {
    match &location {
        TabLocationData::NativeHomePage => "https://www.google.com",
        TabLocationData::WebPage { url } => url.as_str(),
        _ => unimplemented!(),
    }
}

pub struct TabEventHandler {
    id: Id<TabMarker>,
    context: BrowserContext,
}

impl EventHandler for TabEventHandler {
    fn on_title_change(&self, title: String) {
        self.context.delegate.on_tab_title_change(self.id, title);
    }
}
