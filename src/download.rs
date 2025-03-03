use crate::error::PyroError;
use reqwest::Error as ReqwestError;
use std::fs::File;
use std::io::copy;
use std::path::Path;

/// Download a file from a URL and save it to a local path
pub async fn download_file(url: &str, output_path: &Path) -> Result<(), PyroError> {
    let response = reqwest::get(url).await?;
    let mut dest = File::create(output_path)?;
    let content = response.bytes().await?;
    copy(&mut content.as_ref(), &mut dest)?;
    Ok(())
}

/// Download and extract a Python version
pub async fn download_python(version: &str, platform: &str, output_dir: &Path) -> Result<(), PyroError> {
    let url = format!(
        "https://www.python.org/ftp/python/{}/Python-{}-{}.tar.xz",
        version, version, platform
    );
    let archive_path = output_dir.join(format!("Python-{}.tar.xz", version));
    download_file(&url, &archive_path).await?;

    // Extract the archive (implementation depends on the archive format)
    // For now, we'll assume the user has `tar` installed
    let status = std::process::Command::new("tar")
        .arg("-xf")
        .arg(&archive_path)
        .arg("-C")
        .arg(output_dir)
        .status()?;

    if !status.success() {
        return Err(PyroError::Other("Failed to extract archive".to_string()));
    }

    Ok(())
}