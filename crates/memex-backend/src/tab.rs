use std::{ops::Deref, path::PathBuf};

use gpui::{App, AppContext, Entity};
use memex_cef::{Browser, Profile, UIThreadMarker, utils::Rect};
use raw_window_handle::RawWindowHandle;
use uuid::Uuid;

use crate::data::{TabData, TabLocationData};

pub struct Tab {
    pub(crate) id: Uuid,
    pub(crate) browser: Browser,
}

impl Tab {
    pub fn new(
        cx: &mut App,
        window: RawWindowHandle,
        rect: Rect,
        id: Uuid,
        mut profile: Profile,
        location: TabLocationData,
    ) -> anyhow::Result<Entity<Self>> {
        let url = match location {
            TabLocationData::WebPage { url } => url,
            TabLocationData::FileViewer { .. } => unimplemented!(),
            TabLocationData::NativeHomePage => unimplemented!(),
        };
        let browser = Browser::new(&mut profile, window, &url, rect)?;

        Ok(cx.new(|_| Self { id, browser }))
    }

    pub fn on_resize(&self, rect: Rect) {
        self.browser.on_resize(rect);
    }

    pub fn is_native_homepage(&self) -> bool {
        // TODO: implement retrieval whether is webpage native.
        false
    }

    pub fn title(&self) -> String {
        self.browser.title().unwrap()
    }

    pub fn on_resize_view(&self, rect: Rect) {
        self.browser.on_resize(rect);
    }

    pub(crate) fn set_hidden(&self, utm: UIThreadMarker, hidden: bool) {
        #[cfg(target_os = "macos")]
        crate::platform_impl::macos::view::set_hidden(utm, &self.browser, hidden);
        #[cfg(not(target_os = "macos"))]
        unimplemented!()
    }

    pub fn to_data(&self) -> TabData {
        TabData {
            id: self.id,
            location: TabLocationData::WebPage {
                url: self.browser.current_url(),
            },
        }
    }
}

impl Deref for Tab {
    type Target = Browser;

    fn deref(&self) -> &Self::Target {
        &self.browser
    }
}

pub enum TabLocation {
    WebPage { url: String },
    FileViewer { path: PathBuf },
}
