use std::collections::HashMap;

use memex_cef::WebViewBounds;

use crate::{
    BrowserContext, Id, Tab, TabMarker, WorkspaceMarker,
    db::{WorkspaceData, WorkspaceIconData},
    fs::FileSystemItem,
};

pub struct Workspace {
    browser_context: BrowserContext,

    id: Id<WorkspaceMarker>,
    name: String,
    icon: WorkspaceIconData,

    tab_order: Vec<Id<TabMarker>>,
    selected_tab: Option<Id<TabMarker>>,
    tabs: HashMap<Id<TabMarker>, Tab>,
    files: Vec<FileSystemItem>,

    is_loaded: bool,
    is_loading: bool,
}

impl Workspace {
    pub fn new(browser_context: BrowserContext, data: WorkspaceData) -> Self {
        Self {
            browser_context,

            id: data.id,
            name: data.name,
            icon: data.icon,

            tab_order: data.tabs,
            selected_tab: data.selected_tab,
            tabs: HashMap::new(),
            files: Vec::new(),

            is_loaded: false,
            is_loading: false,
        }
    }

    pub fn id(&self) -> Id<WorkspaceMarker> {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn icon(&self) -> &WorkspaceIconData {
        &self.icon
    }

    pub fn tab_order(&self) -> &Vec<Id<TabMarker>> {
        &self.tab_order
    }

    pub fn selected_tab(&self) -> Option<Id<TabMarker>> {
        self.selected_tab
    }

    pub fn tabs(&self) -> &HashMap<Id<TabMarker>, Tab> {
        &self.tabs
    }

    pub fn tabs_mut(&self) -> &mut HashMap<Id<TabMarker>, Tab> {
        &mut self.tabs
    }

    pub fn files(&self) -> &Vec<FileSystemItem> {
        &self.files
    }

    pub fn is_loaded(&self) -> bool {
        self.is_loaded
    }

    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    pub fn resize_webview(&self, bounds: WebViewBounds) {
        for tab in self.tabs.values() {
            if let Some(webview) = tab.webview() {
                webview.resize(bounds);
            }
        }
    }

    pub async fn load(&mut self) -> anyhow::Result<()> {
        anyhow::ensure!(
            !self.is_loaded,
            "このワークスペースは既に読み込み済みです。"
        );

        self.browser_context
            .delegate
            .on_workspace_loading_start(self.id);
        self.is_loading = true;

        // Load files.
        let path = self.browser_context.data.workspace_dir(self.id);
        self.files = crate::fs::build_file_tree(&path).await?;

        // Load tabs.
        for tab in self.tabs.values_mut() {
            tab.load()?;
        }

        self.is_loaded = true;
        self.is_loading = false;
        self.browser_context.delegate.on_workspace_load(self.id);

        Ok(())
    }
}
