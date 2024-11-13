use crate::response::AnyResult;
use rust_utils::log::Log;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;

pub fn file_put_contents(file_path: impl AsRef<Path>, data: &mut [u8]) -> AnyResult<()> {
    log::trace!("file_put_contents");

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_path)
        .log_error("backend::file::save_dile", "Error opening file")?;

    Ok(file
        .write_all(data)
        .log_error("backend::file::save_dile", "Error writing to file")?)
}

pub fn file_get_contents(file_path: impl AsRef<Path>) -> AnyResult<Vec<u8>> {
    log::trace!("file_get_contents");

    let mut file = OpenOptions::new()
        .read(true)
        .open(file_path)
        .log_error("backend::file::get_dile", "Error opening file")?;

    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .log_error("backend::file::get_dile", "Error reading file")?;

    Ok(data)
}
