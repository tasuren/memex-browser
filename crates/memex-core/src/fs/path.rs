use std::{
    path::PathBuf,
    sync::atomic::{AtomicBool, Ordering},
};

use anyhow::Context as _;

use crate::{Id, WorkspaceMarker, fs::utils};

pub static DIRECTORY_CHECKED: AtomicBool = AtomicBool::new(false);

pub(super) async fn setup_data_directory(context: &DataContext) -> anyhow::Result<()> {
    if DIRECTORY_CHECKED.swap(true, Ordering::Relaxed) {
        anyhow::bail!("既にデータディレクトリのセットアップは行われています。");
    }

    if !utils::exists(context.data_local_dir()).await? {
        async_fs::create_dir_all(context.data_local_dir())
            .await
            .context("データディレクトリの作成に失敗しました。")?;
    }

    if !utils::exists(context.chromium_data_dir()).await? {
        async_fs::create_dir(context.chromium_data_dir())
            .await
            .context("ワークスペースディレクトリの作成に失敗しました。")?;
    }

    if !utils::exists(context.workspace_list_dir()).await? {
        async_fs::create_dir(context.workspace_list_dir())
            .await
            .context("ワークスペースディレクトリの作成に失敗しました。")?;
    }

    Ok(())
}

#[cfg(not(debug_assertions))]
struct PathState {
    application_identifier: &'static str,
}

/// アプリのデータを保存するパスを容易するための構造体。
#[derive(Clone)]
pub struct DataContext {
    #[cfg(not(debug_assertions))]
    state: Arc<PathState>,
}

impl DataContext {
    pub fn new(_application_identifier: &'static str) -> Self {
        Self {
            #[cfg(not(debug_assertions))]
            state: Arc::new(PathState {
                application_identifier: _application_identifier,
            }),
        }
    }

    pub fn data_local_dir(&self) -> PathBuf {
        #[cfg(debug_assertions)]
        {
            PathBuf::new().join(".dev").join("app_data")
        }
        #[cfg(not(debug_assertions))]
        {
            dirs::data_local_dir()
                .expect("Failed to get the local data directory path.")
                .join(&self.state.application_identifier)
        }
    }

    pub fn chromium_data_dir(&self) -> PathBuf {
        self.data_local_dir().join("chromium")
    }

    pub fn workspace_list_dir(&self) -> PathBuf {
        self.data_local_dir().join("workspaces")
    }

    pub fn workspace_dir(&self, id: Id<WorkspaceMarker>) -> PathBuf {
        self.workspace_list_dir().join(id.to_string())
    }
}
