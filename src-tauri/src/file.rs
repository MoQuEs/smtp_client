#![allow(dead_code)]

use crate::response::AnyResult;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;

pub fn file_put_contents(file_path: impl AsRef<Path>, data: &mut [u8]) -> AnyResult<()> {
    log::trace!("file_put_contents {file_path}");

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_path)
        .inspect_err(|e| log::error!("Error opening file '{:?}'"))?;

    Ok(file
        .write_all(data)
        .inspect_err(|e| log::error!("Error writing to file '{:?}'"))?)
}

pub fn file_get_contents(file_path: impl AsRef<Path>) -> AnyResult<Vec<u8>> {
    log::trace!("file_get_contents {file_path}");

    let mut file = OpenOptions::new()
        .read(true)
        .open(file_path)
        .inspect_err(|e| log::error!("Error opening file '{:?}'"))?;

    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .inspect_err(|e| log::error!("Error reading file '{:?}'"))?;

    Ok(data)
}

pub fn file_exists(file_path: impl AsRef<Path>) -> bool {
    log::trace!("file_exists {file_path}");

    file_path.as_ref().exists()
}

pub fn file_delete(file_path: impl AsRef<Path>) -> AnyResult<()> {
    log::trace!("file_delete {file_path}");

    if file_exists(&file_path) {
        std::fs::remove_file(&file_path)
            .inspect_err(|e| log::error!("Error deleting file '{:?}'", file_path.as_ref()))?;
    }

    Ok(())
}

pub fn file_rename(from: impl AsRef<Path>, to: impl AsRef<Path>) -> AnyResult<()> {
    log::trace!("file_rename from: {from} to: {to}");

    std::fs::rename(&from, &to).inspect_err(|e| {
        log::error!(
            "Error renaming file from '{:?}' to '{:?}'",
            from.as_ref(),
            to.as_ref()
        )
    })?;

    Ok(())
}
