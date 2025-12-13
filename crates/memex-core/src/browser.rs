use std::{cell::Cell, collections::HashMap, rc::Rc};

use anyhow::Context;
use memex_cef::Profile;
use raw_window_handle::RawWindowHandle;

use crate::{Id, TabMarker, Workspace, WorkspaceMarker, db::Database, fs::DataContext};

pub use memex_cef::WebViewBounds;

#[derive(Clone)]
pub struct BrowserContext {
    pub(crate) data: DataContext,
    pub(crate) db: Database,
    pub(crate) delegate: SharedBrowserDelegate,
    pub(crate) profile: Profile,
    pub(crate) window_handle: RawWindowHandle,
    pub rect: Rc<Cell<WebViewBounds>>,
}

impl BrowserContext {
    pub fn new(
        data: DataContext,
        db: Database,
        window_handle: RawWindowHandle,
        bounds: WebViewBounds,
        delegate: impl BrowserDelegate + 'static,
    ) -> anyhow::Result<Self> {
        let profile_path = data.chromium_data_dir();
        let profile = Profile::new(&profile_path).context("プロファイルの作成に失敗しました。")?;

        Ok(Self {
            data,
            db,
            delegate: Rc::new(delegate),
            profile,
            window_handle,
            rect: Rc::new(Cell::new(bounds)),
        })
    }
}

pub struct Browser {
    context: BrowserContext,
    workspaces: HashMap<Id<WorkspaceMarker>, Workspace>,
    selected_workspace: Id<WorkspaceMarker>,
}

impl Browser {
    pub(crate) fn new(context: BrowserContext) -> anyhow::Result<Self> {
        Ok(Self {
            context,
            workspaces: HashMap::new(),
            selected_workspace: Id::home(),
        })
    }

    pub fn resize_webview(&self, rect: WebViewBounds) {
        self.context.rect.set(rect);

        for workspace in self.workspaces.values() {
            workspace.resize_webview(rect);
        }
    }

    pub async fn select(&mut self, id: Id<WorkspaceMarker>) -> anyhow::Result<()> {
        if let Some(workspace) = self.workspaces.get_mut(&id) {
            self.selected_workspace = id;

            if !workspace.is_loaded() && !workspace.is_loading() {
                workspace
                    .load()
                    .await
                    .context("ワークスペースの読み込みに失敗しました。")?;
            }
        } else {
            anyhow::bail!("そのワークスペースは存在しません。");
        }

        Ok(())
    }

    pub fn add_workspace(&mut self, workspace: Workspace) {
        self.workspaces.insert(workspace.id(), workspace);
    }
}

pub type SharedBrowserDelegate = Rc<dyn BrowserDelegate>;

pub trait BrowserDelegate {
    fn on_workspace_loading_start(&self, id: Id<WorkspaceMarker>);

    fn on_workspace_load(&self, id: Id<WorkspaceMarker>);

    fn on_tab_title_change(&self, id: Id<TabMarker>, title: String);
}
