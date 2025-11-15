use anyhow::Context;
use gpui::{App, Global};
use memex_cef::CefContext;

use crate::data::AppPath;

pub async fn init_global_state(cx: &mut App, application_identifier: String) -> anyhow::Result<()> {
    let cef = CefState::new(CefContext::new());
    let path = AppPath::new(application_identifier)
        .await
        .context("データの保存場所の用意に失敗しました。")?;

    cx.set_global(cef);
    cx.set_global(path);

    Ok(())
}

pub struct CefState {
    _cef: CefContext,
}
impl CefState {
    fn new(cef: CefContext) -> Self {
        Self { _cef: cef }
    }
}

impl Global for CefState {}
impl Global for AppPath {}
