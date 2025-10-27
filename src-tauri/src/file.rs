#![allow(dead_code)]

use crate::dialogs::blocking::pick_file_dialog;
use crate::response::{error, AnyResult};
use crate::state::AppHandle;
use mime_guess::MimeGuess;
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tauri_plugin_dialog::FilePath;

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

pub struct FileData {
    pub path: String,
    pub name: String,
    pub extension: String,
    pub mime: String,
    pub size: u32,
    pub binary: Vec<u8>,
}

pub fn get_file_data_from_dialog(
    app_handle: &AppHandle,
    filter: Vec<(impl AsRef<str> + Debug, &[&str])>,
) -> Result<FileData, String> {
    let file_path = pick_file_dialog(app_handle, filter);
    if file_path.is_none() {
        log::info!("No file selected");
        return Err("No file selected".to_string());
    }

    let file_path = file_path.unwrap();
    log::info!("Opening file: {:?}", file_path);

    let path = match file_path.as_path() {
        Some(p) => p.into_path_buf(),
        None => {
            return Err("Invalid file path".to_string());
        }
    };

    let name = path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "ERROR".to_string());

    let extension = path
        .extension()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "ERROR".to_string());

    let mime = match mime_guess::from_path(&path).first() {
        Some(m) => m.to_string(),
        None => "application/octet-stream".to_string(),
    };

    let binary = match file_get_contents(&path) {
        Ok(b) => b,
        Err(e) => {
            return Err(format!("Failed to read file: {}", e));
        }
    };

    Ok(FileData {
        path: path.to_string(),
        name,
        extension,
        mime,
        size: binary.len() as u32,
        binary,
    })
}

pub fn get_file_data_from_request(url: String) -> Result<FileData, String> {
    if url.is_empty() {
        return Err("URL is empty".to_string());
    }

    let path = url;
    let url_parsed = match url::Url::parse(&path) {
        Ok(u) => u,
        Err(e) => {
            return Err(format!("Invalid URL: {}", e));
        }
    };

    let response = match reqwest::blocking::get(url_parsed.clone()) {
        Ok(r) => r,
        Err(e) => {
            return Err(format!("Failed to get URL: {}", e));
        }
    };

    if !response.status().is_success() {
        return Err(format!("Failed to fetch URL: HTTP {}", response.status()));
    }

    let name = response
        .headers()
        .get(reqwest::header::CONTENT_DISPOSITION)
        .and_then(|cd| cd.to_str().ok())
        .and_then(|cd| {
            let cd_str = cd.to_string();
            let parts: Vec<&str> = cd_str.split("filename=").collect();
            if parts.len() > 1 {
                Some(parts[1].trim_matches('"').to_string())
            } else {
                None
            }
        })
        .or_else(|| {
            url_parsed
                .path_segments()
                .and_then(|segments| segments.last())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| "downloaded_file".to_string());

    let extension = Path::new(&name)
        .extension()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "bin".to_string());

    let mime = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|ct| ct.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_string();

    let binary = match response.bytes() {
        Ok(b) => b.to_vec(),
        Err(e) => {
            return Err(format!("Failed to read response body: {}", e));
        }
    };

    Ok(FileData {
        path,
        name,
        extension,
        mime,
        size: binary.len() as u32,
        binary,
    })
}
