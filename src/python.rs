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

        let document = Html::parse_document(&body);

        // Selector for the container that holds the version list
        let container_selector = Selector::parse("div.download-list-widget").unwrap();
        let container = document.select(&container_selector).next().ok_or_else(|| {
            PyroError::Custom("Could not find the version container on the page.".to_string())
        })?;

        // Selector for the nested `ol` tag within the container
        let ol_selector = Selector::parse("ol.list-row-container.menu").unwrap();
        let ol = container.select(&ol_selector).next().ok_or_else(|| {
            PyroError::Custom("Could not find the version list within the container.".to_string())
        })?;

        // Selector for the `li` tags within the `ol`
        let li_selector = Selector::parse("li").unwrap();
        let version_selector = Selector::parse("span.release-number").unwrap();

        let mut versions = Vec::new();

        for li in ol.select(&li_selector) {
            if let Some(version_span) = li.select(&version_selector).next() {
                let version = version_span.text().collect::<String>().trim().to_string();
                let url = format!("https://www.python.org/downloads/release/{}/", version);
                versions.push(PythonVersion { version, url });
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