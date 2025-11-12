use std::sync::{Arc, Mutex};

use anyhow::Context;
use gpui::{BackgroundExecutor, ForegroundExecutor, Task};
use memex_cef::CefContext;

use crate::{WorkspaceManager, data::AppPath};

#[derive(Clone)]
pub struct SystemContext {
    cef: CefContext,
    path: AppPath,
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    workspace_manager: Option<Arc<Mutex<WorkspaceManager>>>,
}

impl SystemContext {
    pub async fn new(
        background_executor: BackgroundExecutor,
        foreground_executor: ForegroundExecutor,
        path: AppPath,
    ) -> anyhow::Result<Self> {
        let mut system = Self {
            cef: CefContext::new(),
            path,
            background_executor,
            foreground_executor,
            workspace_manager: None,
        };

        let workspace_manager = WorkspaceManager::new(system.clone())
            .await
            .context("ワークスペースマネージャの用意に失敗しました。")?;
        system.workspace_manager = Some(Arc::new(Mutex::new(workspace_manager)));

        Ok(system)
    }

    pub fn cef(&self) -> &CefContext {
        &self.cef
    }

    pub fn path(&self) -> &AppPath {
        &self.path
    }

    pub fn workspace_manager(&self) -> &Arc<Mutex<WorkspaceManager>> {
        self.workspace_manager.as_ref().unwrap()
    }

    pub fn spawn_background<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where
        R: Send + 'static,
    {
        self.background_executor.spawn(future)
    }

    pub fn spawn_foreground<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where
        R: Send + 'static,
    {
        self.foreground_executor.spawn(future)
    }
}

impl gpui::Global for SystemContext {}
