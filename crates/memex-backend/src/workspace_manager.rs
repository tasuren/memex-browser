use std::collections::HashMap;

use anyhow::Context;
use memex_cef::UIThreadMarker;
use raw_window_handle::RawWindowHandle;
use uuid::Uuid;

use crate::{
    SystemContext, Workspace,
    data::{WorkspaceIconData, WorkspaceMetadata, get_manager_state, list_workspace},
};

pub struct WorkspaceManager {
    cx: SystemContext,
    home: Uuid,
    order: Vec<Uuid>,
    workspaces: HashMap<Uuid, WorkspaceMetadata>,
    loaded: HashMap<Uuid, Workspace>,
    selected: Uuid,
}

impl WorkspaceManager {
    pub async fn new(cx: SystemContext) -> anyhow::Result<Self> {
        let (data, created) = get_manager_state(cx.path())
            .await
            .context("WorkspaceManagerのセーブデータ読み込みに失敗しました。")?;
        let workspaces = list_workspace(cx.path())
            .await
            .context("ワークスペースの一覧取得に失敗しました。")?;

        let mut manager = Self {
            cx: cx.clone(),
            home: data.home,
            order: data.order,
            workspaces,
            loaded: HashMap::new(),
            selected: data.selected,
        };

        // 新しくWorkspaceManagerのセーブデータが作られたなら、
        // ホームワークスペースを自動で作成する。
        if created {
            let home = Workspace::new(cx, data.home, "Home".to_owned(), WorkspaceIconData::Home)
                .await
                .context("ホームワークスペースの作成に失敗しました。")?;
            manager.add(home).unwrap();
        }

        Ok(manager)
    }

    pub fn order(&self) -> &Vec<Uuid> {
        &self.order
    }

    pub fn list_metadata(&self) -> &HashMap<Uuid, WorkspaceMetadata> {
        &self.workspaces
    }

    pub fn home(&self) -> Uuid {
        self.home
    }

    pub fn selected(&self) -> Uuid {
        self.selected
    }

    pub async fn open(
        &mut self,
        utm: UIThreadMarker,
        id: Uuid,
        window: RawWindowHandle,
    ) -> anyhow::Result<&mut Workspace> {
        self.selected = id;

        if !self.loaded.contains_key(&id) {
            // まだロードしていないなら、ロードする。
            let workspace = Workspace::load(utm, self.cx.clone(), window, id)
                .await
                .context("ワークスペースの読み込みに失敗しました。")?;
            self.loaded.insert(id, workspace);
        };

        Ok(self.loaded.get_mut(&id).unwrap())
    }

    pub fn get(&self, id: Uuid) -> Option<&Workspace> {
        self.loaded.get(&id)
    }

    pub fn get_mut(&mut self, id: Uuid) -> anyhow::Result<&mut Workspace> {
        self.loaded
            .get_mut(&id)
            .context("そのワークスペースはまだロードしていません。")
    }

    pub fn add(&mut self, workspace: Workspace) -> anyhow::Result<()> {
        anyhow::ensure!(
            self.order.contains(&workspace.id),
            "既にそのワークスペースは追加されています。"
        );

        let metadata = WorkspaceMetadata::from(&workspace);
        self.order.insert(0, workspace.id);
        self.workspaces.insert(workspace.id, metadata);
        self.loaded.insert(workspace.id, workspace);

        Ok(())
    }

    pub fn remove(&mut self, id: Uuid) -> anyhow::Result<()> {
        anyhow::ensure!(
            self.home == id,
            "ホームワークスペースを削除することはできません。"
        );
        anyhow::ensure!(
            self.order.contains(&id),
            "そのワークスペースはマネージャに追加されていません。"
        );

        let pos = self.order.iter().position(|i| *i == id).unwrap();
        self.order.remove(pos);
        self.workspaces.remove(&id);
        self.loaded.remove(&id);

        Ok(())
    }
}
