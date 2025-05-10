use std::{collections::HashMap, path::PathBuf};

use anyhow::bail;
use console::Term;
use isolang::Language;

use crate::utils::{decompress_gzip, download_with_progress, hash_url, read_file, write_file};

pub struct FrequencyMap {
    map: HashMap<String, u32>,
}

const BASE_URL: &str = "https://object.pouta.csc.fi/OPUS-OpenSubtitles/v2024/freq/";

impl FrequencyMap {
    pub async fn new(language: &str, term: &Term) -> anyhow::Result<Option<Self>> {
        if let Some(lang) = Language::from_639_3(language).and_then(|l| l.to_639_1()) {
            let url = format!("{}{}.freq.gz", BASE_URL, lang);
            let file_path = PathBuf::from(".data").join(&hash_url(&url));

            let content = match read_file(&file_path)? {
                Some(content) => {
                    term.write_line(&format!(
                        "✅ Using cached frequency list from {}",
                        file_path.display()
                    ))?;
                    content
                }
                None => {
                    term.write_line(
                        format!("⬇️ Downloading frequency list from {}...", url).as_str(),
                    )?;

                    let content = download_with_progress(&url, &file_path).await?;

                    term.clear_line()?;
                    term.write_line("✅ Download complete")?;

                    write_file(&file_path, &content)?;

                    content
                }
            };

            let mut map = HashMap::new();

            let decoded = String::from_utf8(decompress_gzip(&content)?)?;

            for line in decoded.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();

                if parts.len() >= 2 {
                    if let Ok(frequency) = parts[0].parse::<u32>() {
                        let word = parts[1].to_string();
                        map.insert(word, frequency);
                    }
                }
            }

            term.write_line(&format!(
                "📊 Loaded {} words with frequency data",
                map.len()
            ))?;

            Ok(Some(Self { map }))
        } else {
            term.write_line(&format!(
                "❌ Couldn't find frequency map for language \"{}\"",
                language
            ))?;
            Ok(None)
        }
    }

    pub fn get_frequency(&self, word: &str) -> Option<u32> {
        self.map.get(word).cloned()
    }
}
