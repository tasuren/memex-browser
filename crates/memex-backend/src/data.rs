pub use models::*;
pub use path::*;
pub use workspace::*;

mod workspace {
    use std::{collections::HashMap, io, str::FromStr};

    use anyhow::Context;
    use async_fs as fs;
    use futures_lite::StreamExt;
    use uuid::Uuid;

    use crate::{
        Workspace,
        data::{AppPath, WorkspaceData, WorkspaceIconData, WorkspaceManagerData},
        os::file_system::{FileSystemItem, build_file_tree},
    };

    pub async fn get_manager_state(path: &AppPath) -> anyhow::Result<(WorkspaceManagerData, bool)> {
        let path = path.workspaces().join("manager.json");

        if path.exists() {
            let raw = fs::read(path).await?;

            Ok((serde_json::from_slice(&raw)?, false))
        } else {
            Ok((WorkspaceManagerData::default(), true))
        }
    }

    pub struct WorkspaceMetadata {
        pub(crate) id: Uuid,
        pub(crate) icon: WorkspaceIconData,
        pub(crate) name: String,
    }

    impl WorkspaceMetadata {
        pub fn id(&self) -> Uuid {
            self.id
        }

        pub fn icon(&self) -> &WorkspaceIconData {
            &self.icon
        }

        pub fn name(&self) -> &String {
            &self.name
        }
    }

    impl From<WorkspaceData> for WorkspaceMetadata {
        fn from(value: WorkspaceData) -> Self {
            Self {
                id: value.id,
                icon: value.icon,
                name: value.name,
            }
        }
    }

    impl From<&Workspace> for WorkspaceMetadata {
        fn from(value: &Workspace) -> Self {
            Self {
                id: value.id,
                icon: value.icon.clone(),
                name: value.name.clone(),
            }
        }
    }

    async fn load_workspace_metadata(
        path: &AppPath,
        id: Uuid,
    ) -> anyhow::Result<WorkspaceMetadata> {
        let base = path.workspaces().join(id.to_string());
        anyhow::ensure!(
            base.exists(),
            "指定されたワークスペースのフォルダはありません。"
        );

        let data: WorkspaceData =
            serde_json::from_slice(&fs::read(base.join("workspace.json")).await?)
                .context("ワークスペースのセーブデータを処理するのに失敗しました。")?;
        Ok(data.into())
    }

    pub async fn list_workspace(
        path: &AppPath,
    ) -> anyhow::Result<HashMap<Uuid, WorkspaceMetadata>> {
        let mut entries = fs::read_dir(path.workspaces()).await?;
        let mut workspaces = HashMap::new();

        while let Some(entry) = entries.try_next().await? {
            if !entry.file_type().await?.is_dir() {
                log::warn!(
                    "ワークスペースのリストを取得中に、フォルダではないものがありました: {}",
                    entry.path().display()
                );
            }

            if let Some(id) = entry
                .path()
                .file_name()
                .and_then(|os_str| os_str.to_str())
                .and_then(|name| Uuid::from_str(name).ok())
            {
                let data = load_workspace_metadata(path, id).await.with_context(|| {
                    format!("{id}のワークスペースのデータ読み込みに失敗しました。")
                })?;

                workspaces.insert(id, data);
            } else {
                log::warn!(
                    "ワークスペースの処理中、ワークスペースでない何かに遭遇しました: {}",
                    entry.path().display()
                );
            }
        }

        Ok(workspaces)
    }

    pub async fn create_workspace(
        path: &AppPath,
        id: Uuid,
        name: String,
    ) -> io::Result<WorkspaceData> {
        let data = WorkspaceData::new(id, name);
        let base = path.workspaces().join(data.id.to_string());

        if base.exists() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "指定されたワークスペースのフォルダには既に何か存在しています。",
            ));
        }

        fs::create_dir(&base).await?;
        fs::create_dir(base.join("knowledges")).await?;

        let workspace_data_path = base.join("workspace.json");
        fs::write(workspace_data_path, serde_json::to_vec(&data).unwrap()).await?;

        Ok(data)
    }

    pub async fn load_workspace(
        path: &AppPath,
        id: Uuid,
    ) -> anyhow::Result<(WorkspaceData, Vec<FileSystemItem>)> {
        let base = path.workspaces().join(id.to_string());

        let workspace_data_path = base.join("workspace.json");
        anyhow::ensure!(
            workspace_data_path.exists(),
            "指定されたワークスペースのフォルダには、ワークスペースのセーブデータがありません。"
        );

        let data: WorkspaceData = serde_json::from_slice(&fs::read(workspace_data_path).await?)
            .context("ワークスペースのセーブデータの処理に失敗しました。")?;

        let workspace_files_dir = base.join("knowledges");
        anyhow::ensure!(
            workspace_files_dir.exists(),
            "指定されたワークスペースのフォルダにナレッジフォルダが見つかりませんでした。",
        );

        Ok((
            data,
            build_file_tree(&workspace_files_dir)
                .await
                .context("ワークスペース内のフォルダのスキャンに失敗しました。")?,
        ))
    }

    pub async fn save_workspace(path: &AppPath, data: &WorkspaceData) -> io::Result<()> {
        let base = path.workspaces().join(data.id.to_string());
        if !base.exists() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "指定されたワークスペースのフォルダがありませんでした。",
            ));
        }

        fs::write(
            base.join("workspace.json"),
            serde_json::to_vec(data).unwrap(),
        )
        .await
    }

    pub async fn delete_workspace(path: &AppPath, id: Uuid) -> io::Result<()> {
        let base = path.workspaces().join(id.to_string());
        if !base.exists() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "指定されたワークスペースは既に存在しません。",
            ));
        }

        fs::remove_dir_all(base).await
    }
}

mod models {
    use std::{collections::HashMap, path::PathBuf};

    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use crate::Workspace;

    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct WorkspaceManagerData {
        pub home: Uuid,
        pub order: Vec<Uuid>,
        pub selected: Uuid,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct TabData {
        pub id: Uuid,
        pub location: TabLocationData,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(tag = "type")]
    pub enum TabLocationData {
        WebPage { url: String },
        FileViewer { path: PathBuf },
        NativeHomePage,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(tag = "type")]
    pub enum WorkspaceIconData {
        Home,
        Emoji(String),
        Text(String),
        Image(PathBuf),
        Default,
    }

    impl Default for WorkspaceIconData {
        fn default() -> Self {
            Self::Default
        }
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct WorkspaceData {
        pub id: Uuid,
        pub name: String,
        pub icon: WorkspaceIconData,
        pub tabs: HashMap<Uuid, TabData>,
        pub tab_order: Vec<Uuid>,
        pub selected: Option<Uuid>,
    }

    impl WorkspaceData {
        pub fn new(id: Uuid, name: String) -> Self {
            Self {
                id,
                name,
                icon: Default::default(),
                tabs: Default::default(),
                tab_order: Default::default(),
                selected: Default::default(),
            }
        }
    }

    impl From<&Workspace> for WorkspaceData {
        fn from(value: &Workspace) -> Self {
            WorkspaceData {
                id: value.id,
                name: value.name.clone(),
                icon: value.icon.clone(),
                tabs: value
                    .tabs
                    .iter()
                    .map(|(id, tab)| (*id, tab.to_data()))
                    .collect(),
                tab_order: value.tab_order.clone(),
                selected: value.selected.clone(),
            }
        }
    }
}

mod path {
    use std::{io, path::PathBuf, sync::Arc};

    use async_fs as fs;

    struct PathState {
        application_identifier: String,
    }

    #[derive(Clone)]
    pub struct AppPath {
        state: Arc<PathState>,
    }

    impl AppPath {
        pub async fn new(application_identifier: String) -> io::Result<Self> {
            let path = Self {
                state: Arc::new(PathState {
                    application_identifier,
                }),
            };

            if !path.data_local_dir().exists() {
                fs::create_dir_all(path.data_local_dir()).await?;
            }

            if !path.workspaces().exists() {
                fs::create_dir(path.workspaces()).await?;
            }

            Ok(path)
        }

        pub fn data_local_dir(&self) -> PathBuf {
            dirs::data_local_dir()
                .expect("Failed to get the local data directory path.")
                .join(&self.state.application_identifier)
        }

        pub fn workspaces(&self) -> PathBuf {
            self.data_local_dir().join("workspaces")
        }
    }
}
