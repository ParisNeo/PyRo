use crate::error::PyroError;
use reqwest::Error as ReqwestError;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PythonVersion {
    pub version: String,
    pub url: String,
}

pub struct PythonManager;

impl PythonManager {
    pub async fn list_versions() -> Result<(), PyroError> {
        let versions = Self::fetch_python_versions().await?;
        for v in versions {
            println!("Version: {}", v.version);
        }
        Ok(())
    }

    pub async fn install_version(version: &str) -> Result<(), PyroError> {
        Self::download_and_install(version).await?;
        Ok(())
    }

    async fn fetch_python_versions() -> Result<Vec<PythonVersion>, ReqwestError> {
        // Use the official Python download page or a reliable API
        let url = "https://www.python.org/downloads/";
        let response = reqwest::get(url).await?;
        let body = response.text().await?;

        // Parse the HTML to extract Python versions (this is a simplified example)
        let versions = vec![
            PythonVersion {
                version: "3.12.0".to_string(),
                url: "https://www.python.org/ftp/python/3.12.0/Python-3.12.0.tar.xz".to_string(),
            },
            PythonVersion {
                version: "3.11.0".to_string(),
                url: "https://www.python.org/ftp/python/3.11.0/Python-3.11.0.tar.xz".to_string(),
            },
            // Add more versions as needed
        ];

        Ok(versions)
    }

    async fn download_and_install(version: &str) -> Result<(), PyroError> {
        let url = format!("https://www.python.org/ftp/python/{}/Python-{}.tar.xz", version, version);
        let response = reqwest::get(&url).await?;
        let mut dest = std::fs::File::create(format!("Python-{}.tar.xz", version))?;
        let content = response.bytes().await?;
        std::io::copy(&mut content.as_ref(), &mut dest)?;
        Ok(())
    }
}