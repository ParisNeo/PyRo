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
    
        // Build a client with a custom User-Agent header
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.93 Safari/537.36")
            .build()?;
    
        let response = client.get(url).send().await?;
        let body = response.text().await?;
    
        let document = Html::parse_document(&body);
    
        // Fixed fallback list
        let fallback = vec![PythonVersion {
            version: "3.11.9".to_string(),
            url: "https://www.python.org/downloads/release/python-31111/".to_string(),
        }];
    
        // Selector for the container that holds the version list
        let container_selector = Selector::parse("div.download-list-widget").unwrap();
        let container = document.select(&container_selector).next();
        if container.is_none() {
            return Ok(fallback);
        }
        let container = container.unwrap();
    
        // Selector for the nested `ol` tag within the container
        let ol_selector = Selector::parse("ol.list-row-container.menu").unwrap();
        let ol = container.select(&ol_selector).next();
        if ol.is_none() {
            return Ok(fallback);
        }
        let ol = ol.unwrap();
    
        // Selector for the `li` tags within the `ol`
        let li_selector = Selector::parse("li").unwrap();
        let version_selector = Selector::parse("span.release-number").unwrap();
    
        let mut versions = Vec::new();
    
        for li in ol.select(&li_selector) {
            if let Some(version_span) = li.select(&version_selector).next() {
                let version = version_span.text().collect::<String>().trim().to_string();
                let version_url = format!("https://www.python.org/downloads/release/{}/", version);
                versions.push(PythonVersion { version, url: version_url });
            }
        }
    
        // Return the scraped versions if any, otherwise return the fallback.
        if versions.is_empty() {
            Ok(fallback)
        } else {
            Ok(versions)
        }
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