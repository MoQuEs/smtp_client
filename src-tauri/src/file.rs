use crate::response::AnyResult;
use rust_utils::log::Log;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn file_put_contents(file_path: impl AsRef<Path>, data: &mut [u8]) -> AnyResult<()> {
    log::trace!("save_dile");

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_path)
        .log_error("backend::file::save_dile", "Error opening file")?;

    Ok(file
        .write_all(data)
        .log_error("backend::file::save_dile", "Error writing to file")?)
}
