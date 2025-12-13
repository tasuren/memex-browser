use std::{io, path::Path};

pub async fn exists(path: impl AsRef<Path>) -> io::Result<bool> {
    Ok(match async_fs::metadata(path).await {
        Ok(_) => true,
        Err(error) if error.kind() == io::ErrorKind::NotFound => false,
        Err(error) => return Err(error),
    })
}
