pub use model::*;

use crate::{db::{Database}, Id, WorkspaceMarker};

pub(super) async fn setup_workspace_table(db: &Database) -> anyhow::Result<()> {
    let home = Id::<WorkspaceMarker>::home();

    sqlx::query!(
        "
        INSERT INTO workspace (id, icon_type)
        VALUES (?, 'Home');
        ",
        *home
    )
    .execute(db)
    .await?;

    Ok(())
}

mod model {
    use std::path::PathBuf;

    use crate::{Id, TabMarker, WorkspaceMarker};

    #[derive(Debug, Default, Clone)]
    pub enum WorkspaceIconData {
        Home,
        Emoji(String),
        Text(String),
        Image(PathBuf),
        #[default]
        Default,
    }

    #[derive(Debug)]
    pub struct WorkspaceData {
        pub id: Id<WorkspaceMarker>,
        pub name: String,
        pub icon: WorkspaceIconData,

        pub tabs: Vec<Id<TabMarker>>,
        pub selected_tab: Option<Id<TabMarker>>,
    }
}
