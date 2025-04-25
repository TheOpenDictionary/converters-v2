use anyhow::Context;
use async_trait::async_trait;
use console::Term;
use reqwest::Client;

use crate::processors::traits::Downloader;

use super::schema::FreeDictApiEntry;

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

    fn url(&self) -> String {
        // Base URL for the FreeDict database JSON
        "https://freedict.org/freedict-database.json".to_string()
    }

    async fn download(&self, term: &Term) -> anyhow::Result<Vec<u8>> {
        // First, download the FreeDict database index
        term.write_line("⬇️ Downloading FreeDict database index...")?;
        let index_data = super::super::traits::Downloader::download(self, term).await?;

        // Parse the JSON data with serde
        let api_entries: Vec<FreeDictApiEntry> = serde_json::from_slice(&index_data)
            .context("Failed to parse FreeDict database index")?;

        // Find the requested dictionary based on language pair
        let client = Client::new();
        let language_pair = self.language_pair.as_deref();

        for entry in &api_entries {
            // Skip if we're looking for a specific language pair and this isn't it
            if let Some(requested_pair) = language_pair {
                if entry.name != requested_pair {
                    continue;
                }
            }

            // Find source releases
            for release in &entry.releases {
                if release.platform == "src" {
                    term.write_line(&format!(
                        "⬇️ Downloading dictionary for language pair: {}",
                        entry.name
                    ))?;

                    // Download the dictionary file
                    let resp = client
                        .get(&release.url)
                        .send()
                        .await
                        .context("Failed to download dictionary")?;

                    if !resp.status().is_success() {
                        anyhow::bail!("Failed to download dictionary: HTTP {}", resp.status());
                    }

                    let dict_data = resp
                        .bytes()
                        .await
                        .context("Failed to read dictionary data")?;

                    return Ok(dict_data.to_vec());
                }
            }
        }

        // If we're looking for a specific dictionary and couldn't find it
        if language_pair.is_some() {
            anyhow::bail!(
                "Dictionary for language pair '{}' not found",
                language_pair.unwrap()
            );
        } else {
            anyhow::bail!("No dictionaries found");
        }
    }
}
