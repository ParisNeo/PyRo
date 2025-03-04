use crate::error::PyroError;
use reqwest::Error as ReqwestError;
use serde::Deserialize;
use scraper::{Html, Selector};
use select::document::Document;
use select::predicate::Attr;
use select::predicate::Class;
use std::error::Error;

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


    async fn fetch_python_versions() -> Result<Vec<PythonVersion>, PyroError> {
        let url = "https://www.python.org/downloads/";
        let response = reqwest::get(url).await?;
        let body = response.text().await?;

        let document = Document::from(body.as_str());
        let version_elements = document.find(Attr("class", "release-number"));

        let mut versions = Vec::new();

        for element in version_elements {
            if let Some(href) = element.find(Attr("href", "/downloads/release/")).next() {
                let version_str = href.text();
                let url = format!("https://www.python.org{}", href.attr("href").unwrap());
                versions.push(PythonVersion {
                    version: version_str,
                    url,
                });
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