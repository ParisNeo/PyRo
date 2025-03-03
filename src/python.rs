use crate::error::PyroError;
use reqwest::Error as ReqwestError;
use serde::Deserialize;
use scraper::{Html, Selector};

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
        let url = "https://www.python.org/downloads/";
        let response = reqwest::get(url).await?;
        let body = response.text().await?;

        let document = Html::parse_document(&body);
        let selector = Selector::parse(".download-list-widget li a").unwrap();

        let mut versions = Vec::new();

        for element in document.select(&selector) {
            if let Some(version) = element.value().attr("href") {
                if version.contains("/ftp/python/") {
                    let version_str = version.split('/').nth(3).unwrap_or_default();
                    let url = format!("https://www.python.org{}", version);
                    versions.push(PythonVersion {
                        version: version_str.to_string(),
                        url,
                    });
                }
            }
        }

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