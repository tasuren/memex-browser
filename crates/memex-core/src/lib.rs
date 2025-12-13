use std::sync::OnceLock;

use memex_cef::CefContext;

use crate::{db::Database, fs::DataContext};

pub use browser::*;
pub use id::*;
pub use tab::*;
pub use workspace::*;

mod browser;
pub mod db;
pub mod fs;
mod id;
mod tab;
mod workspace;

pub async fn setup_application_data(
    application_identifier: &'static str,
) -> anyhow::Result<(DataContext, Database)> {
    let data_context = DataContext::new(application_identifier);

    fs::setup_fs(&data_context).await?;
    let db = db::setup_database(&data_context).await?;

    Ok((data_context, db))
}

pub static CEF_CONTEXT: OnceLock<CefContext> = OnceLock::new();

pub fn setup_chromium(
    data_context: DataContext,
    product_name: &str,
    locale: &str,
) -> anyhow::Result<()> {
    if CEF_CONTEXT.get().is_some() {
        anyhow::bail!("2度`setup_chromium`を呼び出すことはできません。")
    }

    let root_cache_path = &data_context.chromium_data_dir();
    memex_cef::boot(root_cache_path, root_cache_path, product_name, locale)?;
    _ = CEF_CONTEXT.set(CefContext::default());

    Ok(())
}
