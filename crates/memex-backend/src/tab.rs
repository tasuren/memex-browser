use std::{ops::Deref, path::PathBuf};

use memex_cef::{Browser, Profile, UIThreadMarker};
use raw_window_handle::RawWindowHandle;
use uuid::Uuid;

use crate::{
    SystemContext,
    data::{TabData, TabLocationData},
};

pub struct Tab {
    pub(crate) id: Uuid,
    pub(crate) browser: Browser,
}

impl Tab {
    pub async fn new(
        id: Uuid,
        cx: SystemContext,
        mut profile: Profile,
        window: RawWindowHandle,
        location: TabLocationData,
    ) -> anyhow::Result<Self> {
        let url = match location {
            TabLocationData::WebPage { url } => url,
            TabLocationData::FileViewer { .. } => unimplemented!(),
            TabLocationData::NativeHomePage => unimplemented!(),
        };

        Ok(Self {
            id,
            browser: Browser::new(cx.cef().clone(), &mut profile, window, &url).await?,
        })
    }

    pub fn is_native_homepage(&self) -> bool {
        // TODO: implement retrieval whether is webpage native.
        false
    }

    pub fn title(&self) -> String {
        self.browser.title().unwrap()
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
