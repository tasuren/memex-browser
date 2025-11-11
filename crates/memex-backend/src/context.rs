use gpui::{BackgroundExecutor, ForegroundExecutor, Task};
use memex_cef::CefContext;

use crate::data::AppPath;

#[derive(Clone)]
pub struct SystemContext {
    cef: CefContext,
    path: AppPath,
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
}

impl SystemContext {
    pub fn new(
        background_executor: BackgroundExecutor,
        foreground_executor: ForegroundExecutor,
        path: AppPath,
    ) -> Self {
        Self {
            cef: CefContext::new(),
            path,
            background_executor,
            foreground_executor,
        }
    }

    pub fn cef(&self) -> &CefContext {
        &self.cef
    }

    pub fn path(&self) -> &AppPath {
        &self.path
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
