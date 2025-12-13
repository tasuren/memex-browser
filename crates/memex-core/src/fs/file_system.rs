use std::{
    io,
    path::{Path, PathBuf},
};

use futures_lite::StreamExt;

/// 特定のファイルまたはフォルダのデータ。
pub enum FileSystemItem {
    File(PathBuf),
    Dir {
        path: PathBuf,
        children: Vec<FileSystemItem>,
    },
}

/// 指定されたディレクトリパスを再帰的にスキャンし、`Vec<FileSystemItem>`を返す。
/// これは`build_file_tree`の内部ヘルパー関数。
async fn scan_recursive(dir_path: &Path) -> Result<Vec<FileSystemItem>, io::Error> {
    let mut items = Vec::new();
    let mut entries = async_fs::read_dir(dir_path).await?;

    while let Some(entry) = entries.try_next().await? {
        let path = entry.path();
        let file_type = entry.file_type().await?;

        if file_type.is_dir() {
            let children = Box::pin(scan_recursive(&path)).await?;

            items.push(FileSystemItem::Dir { path, children });
        } else if file_type.is_file() {
            items.push(FileSystemItem::File(path));
        }
    }

    Ok(items)
}

/// ファイルツリーを作る。
pub async fn build_file_tree(root_path: &Path) -> Result<Vec<FileSystemItem>, io::Error> {
    if !root_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "指定されたパスがフォルダではありません。",
        ));
    }

    scan_recursive(root_path).await
}
