use std::collections::HashMap;

use anyhow::Context;
use memex_cef::{Profile, UIThreadMarker};
use raw_window_handle::HasWindowHandle;
use uuid::Uuid;

use crate::{
    SystemContext,
    data::{
        WorkspaceData, delete_workspace, get_workspaces, load_workspace, prepare_workspace,
        save_workspace,
    },
    os::file_system::FileSystemItem,
    tab::Tab,
};

pub struct Workspace {
    cx: SystemContext,
    _profile: Profile,

    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) tabs: HashMap<Uuid, Tab>,
    pub(crate) tab_order: Vec<Uuid>,
    pub(crate) selected: Option<Uuid>,

    files: Vec<FileSystemItem>,
}

impl Workspace {
    pub async fn create(cx: SystemContext, name: String) -> anyhow::Result<Self> {
        let data = prepare_workspace(cx.path(), name.clone())
            .await
            .context("Failed to prepare the workspace directory.")?;

        Ok(Self {
            cx,
            _profile: Profile::new().context("Failed to prepare profile.")?,
            id: data.id,
            name: data.name,
            tabs: HashMap::new(),
            tab_order: Vec::new(),
            selected: None,
            files: Vec::new(),
        })
    }

    pub async fn load(
        cx: SystemContext,
        window: &impl HasWindowHandle,
        id: Uuid,
    ) -> anyhow::Result<Self> {
        let (data, files) = load_workspace(cx.path(), id)
            .await
            .context("Failed to load the workspace.")?;

        let profile = Profile::new().context("Failed to prepare the profile.")?;

        let mut tabs = HashMap::new();
        for (id, tab_data) in data.tabs.into_iter() {
            let tab = Tab::new(id, cx.clone(), profile.clone(), window, tab_data.location)
                .await
                .with_context(|| format!("Failed to restore the tab {}.", id))?;

            tabs.insert(id, tab);
        }

        let workspace = Self {
            cx,
            _profile: profile,
            id,
            name: data.name,
            tabs,
            tab_order: data.tab_order,
            selected: None,
            files,
        };

        Ok(workspace)
    }

    pub async fn load_all(
        cx: SystemContext,
        window: &impl HasWindowHandle,
    ) -> anyhow::Result<Vec<Workspace>> {
        let mut list = Vec::new();

        for id in get_workspaces(cx.path())
            .await
            .context("Failed to get workspaces.")?
            .into_iter()
        {
            list.push(
                Self::load(cx.clone(), window, id)
                    .await
                    .with_context(|| format!("Failed to load the workspace {}.", id))?,
            );
        }

        Ok(list)
    }

    pub fn try_save(&self) {
        self.cx
            .spawn_background({
                let path = self.cx.path().clone();
                let data: WorkspaceData = self.into();

                async move {
                    let id = data.id;

                    if let Err(e) = save_workspace(&path, &data).await {
                        log::warn!("Failed to save the workspace data of {}: {}", id, e);
                    };
                }
            })
            .detach();
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub async fn set_name(&mut self, name: String) {
        self.name = name;
        self.try_save();
    }

    pub fn create_tabe(&mut self, tab: Tab) {
        self.tabs.insert(tab.id, tab);
        self.try_save();
    }

    pub fn close_tab(&mut self, utm: UIThreadMarker, id: Uuid) -> anyhow::Result<()> {
        let tab = self.tabs.remove(&id).context("This tab is not found.")?;
        self.try_save();
        tab.close(utm)
            .context("Failed to properly close the tab.")?;

        Ok(())
    }

    pub async fn destroy(mut self, utm: UIThreadMarker) -> anyhow::Result<()> {
        for (id, tab) in self.tabs.drain() {
            tab.close(utm)
                .with_context(|| format!("Failed to close the tab {}.", id))?;
        }

        delete_workspace(self.cx.path(), self.id)
            .await
            .context("Failed to delete the workspace data.")?;

        Ok(())
    }

    pub fn files(&self) -> &[FileSystemItem] {
        &self.files
    }
}
