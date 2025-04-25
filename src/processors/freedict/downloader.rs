use anyhow::Context;
use async_trait::async_trait;
use serde_json::from_slice;

use crate::processors::{freedict::schema::database::Platform, traits::Downloader};

pub struct FreeDictDownloader {
    language_pair: Option<String>,
}

#[async_trait(?Send)]
impl Downloader for FreeDictDownloader {
    fn new(language_pair: Option<String>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self { language_pair })
    }

    async fn url(&self) -> anyhow::Result<String> {
        let database = reqwest::get("https://freedict.org/freedict-database.json")
            .await?
            .text()
            .await?;

        let json: Vec<super::schema::database::WelcomeElement> =
            from_slice(database.as_bytes()).context("Failed to parse JSON")?;

        for entry in &json {
            if entry.name == self.language_pair {
                if let Some(release) = entry
                    .releases
                    .as_ref()
                    .unwrap()
                    .iter()
                    .find(|e| e.platform == Platform::Src)
                {
                    return Ok(release.url.clone());
                }
            }
        }

        anyhow::bail!("No matching language pair found in the database");
    }
}
