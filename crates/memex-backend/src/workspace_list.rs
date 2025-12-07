use std::collections::HashMap;

use gpui::{App, Entity, Window, prelude::*};
use uuid::Uuid;

use crate::{
    LayoutState, WorkspaceState,
    data::{WorkspaceData, WorkspaceListData, WorkspaceMetadata},
    os::file_system::FileSystemItem,
};

pub struct WorkspaceListState {
    pub layout_state: Entity<LayoutState>,

    home: Uuid,
    order: Vec<Uuid>,
    workspaces: HashMap<Uuid, WorkspaceMetadata>,
    loaded: HashMap<Uuid, Entity<WorkspaceState>>,
    selected: Uuid,
}

impl WorkspaceListState {
    pub fn new(
        window: &mut Window,
        cx: &mut App,
        layout_state: Entity<LayoutState>,
        data: WorkspaceListData,
        workspaces: HashMap<Uuid, WorkspaceMetadata>,
        selected_workspace: WorkspaceData,
        selected_workspace_fields: Vec<FileSystemItem>,
    ) -> anyhow::Result<Entity<Self>> {
        anyhow::ensure!(
            !workspaces.is_empty(),
            "少なくともワークスペースは一つ以上存在していなければなりません。\
            ホームワークスペースを追加してください。"
        );
        anyhow::ensure!(
            workspaces.contains_key(&data.home),
            "セーブデータにホームワークスペースがありませんでした。追加してください。"
        );
        anyhow::ensure!(
            selected_workspace.id == data.selected,
            "選択済みのワークスペースのデータが渡されませんでした。"
        );
        anyhow::ensure!(
            selected_workspace.id == data.selected,
            "選択済みのワークスペースのデータが渡されませんでした。"
        );

        let rect = layout_state.read(cx).view_rect(window);
        let list = cx.new(|_cx| Self {
            layout_state,

            home: data.home,
            order: data.order,
            workspaces,
            loaded: HashMap::new(),
            selected: data.selected,
        });

        // 前回選択されていたワークスペースを読み込んでおく。
        list.update(cx, |list, cx| {
            let workspace = WorkspaceState::new(
                window,
                cx,
                rect,
                selected_workspace,
                selected_workspace_fields,
            )
            .unwrap();
            list.loaded.insert(data.selected, workspace);
        });

        Ok(list)
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

    pub fn current(&self) -> &Entity<WorkspaceState> {
        self.loaded.get(&self.selected).expect(
            "何かしら選択されているはずなのに、ワークスペースがロードされていませんでした。",
        )
    }

    fn show_workspace_tabs(&self, cx: &mut App, id: Uuid) {
        for workspace in self.loaded.values() {
            workspace.update(cx, |workspace, cx| {
                let hidden = workspace.id() != id;
                workspace.set_hidden(cx, hidden);
            });
        }
    }

    pub fn select(&mut self, cx: &mut App, id: Uuid) -> anyhow::Result<()> {
        self.selected = id;
        anyhow::ensure!(
            self.loaded.contains_key(&id),
            "そのワークスペースはまだロードされていません。"
        );

        self.show_workspace_tabs(cx, id);

        Ok(())
    }

    pub fn get(&self, id: &Uuid) -> Option<&Entity<WorkspaceState>> {
        self.loaded.get(id)
    }

    pub fn is_loaded(&self, id: Uuid) -> bool {
        self.loaded.contains_key(&id)
    }

    pub fn load(&mut self, cx: &mut Context<Self>, workspace: Entity<WorkspaceState>) {
        self.loaded.insert(workspace.read(cx).id, workspace);
    }

    pub fn add(
        &mut self,
        cx: &mut Context<Self>,
        workspace: Entity<WorkspaceState>,
    ) -> anyhow::Result<()> {
        let id = {
            let workspace = workspace.read(cx);
            anyhow::ensure!(
                !self.order.contains(&workspace.id),
                "既にそのワークスペースは追加されています。"
            );

            let metadata = WorkspaceMetadata::from(workspace);
            self.order.insert(0, workspace.id);
            self.workspaces.insert(workspace.id, metadata);

            workspace.id
        };

        self.loaded.insert(id, workspace);

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
