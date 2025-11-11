use std::{io, str::FromStr};

use async_fs as fs;
use futures_lite::StreamExt;

use crate::os::file_system::{FileSystemItem, build_file_tree};

pub use models::*;
pub use path::AppPath;
use uuid::Uuid;

pub async fn get_workspaces(path: &AppPath) -> io::Result<Vec<Uuid>> {
    let mut entries = fs::read_dir(path.workspaces()).await?;
    let mut workspaces = Vec::new();

    while let Some(entry) = entries.try_next().await? {
        if !entry.file_type().await?.is_dir() {
            log::warn!(
                "During getting workspace list, I got non-directory one: {}",
                entry.path().display()
            );
        }

        if let Some(id) = entry
            .path()
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .and_then(|name| Uuid::from_str(name).ok())
        {
            workspaces.push(id);
        } else {
            log::warn!(
                "During getting workspace list, I got non-workspace directory: {}",
                entry.path().display()
            );
        }
    }

    Ok(workspaces)
}

pub async fn prepare_workspace(path: &AppPath, name: String) -> io::Result<WorkspaceData> {
    let data = WorkspaceData::new(name);
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
) -> io::Result<(WorkspaceData, Vec<FileSystemItem>)> {
    let base = path.workspaces().join(id.to_string());

    let workspace_data_path = base.join("workspace.json");
    if !workspace_data_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "指定されたワークスペースのフォルダには、ワークスペースのセーブデータがありません。",
        ));
    };

    let data: WorkspaceData =
        serde_json::from_slice(&fs::read(workspace_data_path).await?).unwrap();

    let workspace_files_dir = base.join("knowledges");
    if !workspace_files_dir.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "指定されたワークスペースのフォルダにナレッジフォルダが見つかりませんでした。",
        ));
    }

    Ok((data, build_file_tree(&workspace_files_dir).await?))
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

mod models {
    use std::{collections::HashMap, path::PathBuf};

    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use crate::Workspace;

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

    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct WorkspaceData {
        pub name: String,
        pub id: Uuid,
        pub tabs: HashMap<Uuid, TabData>,
        pub tab_order: Vec<Uuid>,
        pub selected: Option<Uuid>,
    }

    impl WorkspaceData {
        pub fn new(name: String) -> Self {
            Self {
                name,
                id: Uuid::new_v4(),
                tabs: Default::default(),
                tab_order: Default::default(),
                selected: Default::default(),
            }
        }
    }

    impl From<&Workspace> for WorkspaceData {
        fn from(value: &Workspace) -> Self {
            WorkspaceData {
                name: value.name.clone(),
                id: value.id,
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
