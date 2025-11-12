use std::collections::HashMap;

use anyhow::Context;
use memex_cef::{Profile, UIThreadMarker};
use raw_window_handle::RawWindowHandle;
use uuid::Uuid;

use crate::{
    SystemContext,
    data::{
        TabLocationData, WorkspaceData, WorkspaceIconData, create_workspace, delete_workspace,
        load_workspace, save_workspace,
    },
    os::file_system::FileSystemItem,
    tab::Tab,
};

pub struct Workspace {
    cx: SystemContext,
    profile: Profile,

    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) icon: WorkspaceIconData,
    pub(crate) tabs: HashMap<Uuid, Tab>,
    pub(crate) tab_order: Vec<Uuid>,
    pub(crate) selected: Option<Uuid>,

    files: Vec<FileSystemItem>,
}

impl Workspace {
    pub async fn new(
        cx: SystemContext,
        id: Uuid,
        name: String,
        icon: WorkspaceIconData,
    ) -> anyhow::Result<Self> {
        let data = create_workspace(cx.path(), id, name.clone())
            .await
            .context("ワークスペースのデータ用意に失敗しました。")?;

        Ok(Self {
            cx,
            profile: Profile::new().context("プロファイルの用意に失敗しました。")?,
            id: data.id,
            name: data.name,
            icon,
            tabs: HashMap::new(),
            tab_order: Vec::new(),
            selected: None,
            files: Vec::new(),
        })
    }

    pub async fn load(
        utm: UIThreadMarker,
        cx: SystemContext,
        window: RawWindowHandle,
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

            tab.set_hidden(utm, true);
            tabs.insert(id, tab);
        }

        let workspace = Self {
            cx,
            profile,
            id,
            name: data.name,
            icon: data.icon,
            tabs,
            tab_order: data.tab_order,
            selected: None,
            files,
        };

        Ok(workspace)
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

    pub fn selected_tab(&self) -> Option<Uuid> {
        self.selected
    }

    pub fn select(&mut self, utm: UIThreadMarker, id: Uuid) -> anyhow::Result<()> {
        for (tab_id, tab) in self.tabs.iter() {
            if *tab_id == id {
                tab.set_hidden(utm, false);
            } else {
                tab.set_hidden(utm, true);
            }
        }

        self.selected = Some(id);

        Ok(())
    }

    pub fn tab_order(&self) -> &Vec<Uuid> {
        &self.tab_order
    }

    pub fn get_tab(&self, id: Uuid) -> Option<&Tab> {
        self.tabs.get(&id)
    }

    pub async fn set_name(&mut self, name: String) {
        self.name = name;
        self.try_save();
    }

    pub async fn create_tab(&mut self, window: RawWindowHandle) -> anyhow::Result<()> {
        let tab = Tab::new(
            Uuid::new_v4(),
            self.cx.clone(),
            self.profile.clone(),
            window,
            TabLocationData::WebPage {
                url: "https://www.google.com".to_owned(),
            },
        )
        .await?;

        self.tabs.insert(tab.id, tab);
        self.try_save();

        Ok(())
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
