pub use file_system::*;
pub use path::*;

mod file_system;
mod path;
pub mod utils;

pub async fn setup_fs(context: &DataContext) -> anyhow::Result<()> {
    setup_data_directory(context).await?;

    Ok(())
}
