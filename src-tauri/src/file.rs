#![allow(dead_code)]

use crate::response::AnyResult;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;

pub fn file_put_contents(file_path: impl AsRef<Path>, data: &mut [u8]) -> AnyResult<()> {
    log::trace!("file_put_contents {}", file_path.as_ref().display());

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_path)
        .inspect_err(|e| log::error!("Error opening file '{e:?}'"))?;

    Ok(file
        .write_all(data)
        .inspect_err(|e| log::error!("Error writing to file '{e:?}'"))?)
}

pub fn file_get_contents(file_path: impl AsRef<Path>) -> AnyResult<Vec<u8>> {
    log::trace!("file_get_contents {}", file_path.as_ref().display());

    let mut file = OpenOptions::new()
        .read(true)
        .open(file_path)
        .inspect_err(|e| log::error!("Error opening file '{e:?}'"))?;

    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .inspect_err(|e| log::error!("Error reading file '{e:?}'"))?;

    Ok(data)
}

pub fn file_exists(file_path: impl AsRef<Path>) -> bool {
    log::trace!("file_exists {}", file_path.as_ref().display());

    file_path.as_ref().exists()
}

pub fn file_delete(file_path: impl AsRef<Path>) -> AnyResult<()> {
    log::trace!("file_delete {}", file_path.as_ref().display());

    if file_exists(&file_path) {
        std::fs::remove_file(&file_path).inspect_err(|e| {
            log::error!(
                "Error deleting file '{}' '{e:?}'",
                file_path.as_ref().display()
            )
        })?;
    }

    Ok(())
}

pub fn file_rename(from: impl AsRef<Path>, to: impl AsRef<Path>) -> AnyResult<()> {
    log::trace!(
        "file_rename from: {} to: {}",
        from.as_ref().display(),
        to.as_ref().display()
    );

    std::fs::rename(&from, &to).inspect_err(|e| {
        log::error!(
            "Error renaming file from '{}' to '{}' {e:?}",
            from.as_ref().display(),
            to.as_ref().display()
        )
    })?;

    Ok(())
}
