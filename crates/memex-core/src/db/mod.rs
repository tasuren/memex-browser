use anyhow::Context as _;
use sqlx::SqlitePool;

use crate::fs::DataContext;

pub use tab::*;
pub use workspace::*;

mod tab;
mod workspace;

pub type Database = SqlitePool;

pub async fn setup_database(context: &DataContext) -> anyhow::Result<Database> {
    let path = context.data_local_dir().join("data.db");
    let url = format!("sqlite:{}", path.display());

    let db = SqlitePool::connect(&url)
        .await
        .context("データベースへの接続に失敗しました。")?;

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .context("データベースのマイグレーションに失敗しました。")?;

    setup_workspace_table(&db)
        .await
        .context("ワークスペースのデータベースでの準備に失敗しました。")?;

    Ok(db)
}
