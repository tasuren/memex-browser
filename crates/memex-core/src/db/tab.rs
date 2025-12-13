pub use model::*;

use crate::{Id, Tab, TabMarker, WorkspaceMarker, db::Database};

pub async fn add_tab(
    db: &Database,
    workspace_id: Id<WorkspaceMarker>,
    tab: &Tab,
) -> anyhow::Result<()> {
    let id = tab.id();

    let location = tab.location();
    let location_type = location.r#type();
    let location_source = location.source();

    sqlx::query!(
        "
        INSERT INTO tab (id, workspace_id, location_type, location_source)
        VALUES (?, ?, ?, ?);
        ",
        *id,
        *workspace_id,
        location_type,
        location_source
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn remove_tab(db: &Database, id: Id<TabMarker>) -> anyhow::Result<()> {
    sqlx::query!("DELETE FROM tab WHERE id = ?;", *id)
        .execute(db)
        .await?;

    Ok(())
}

pub async fn update_location(
    db: &Database,
    id: Id<TabMarker>,
    location: &TabLocationData,
) -> anyhow::Result<()> {
    let location_type = location.r#type();
    let location_source = location.source();

    sqlx::query!(
        "UPDATE tab SET location_type = ?, location_source = ? WHERE id= ?;",
        location_type,
        location_source,
        *id
    )
    .execute(db)
    .await?;

    Ok(())
}

mod model {
    use std::path::PathBuf;

    use crate::{Id, TabMarker};

    #[derive(Clone, Debug)]
    pub enum TabLocationData {
        WebPage { url: String },
        FileViewer { path: PathBuf },
        NativeHomePage,
    }

    impl TabLocationData {
        pub fn r#type(&self) -> &'static str {
            match self {
                Self::WebPage { .. } => "WebPage",
                Self::FileViewer { .. } => "FileViewer",
                Self::NativeHomePage => "NativeHomePage",
            }
        }

        pub fn source(&self) -> Option<String> {
            match self {
                Self::WebPage { url } => Some(url.clone()),
                Self::FileViewer { path } => {
                    Some(path.to_str().expect("パスの文字列化に失敗").to_owned())
                }
                Self::NativeHomePage => None,
            }
        }
    }

    #[derive(Debug)]
    pub struct TabData {
        pub id: Id<TabMarker>,
        pub location: TabLocationData,
    }
}
