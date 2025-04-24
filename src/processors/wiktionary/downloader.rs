use crate::processors::traits::Downloader;

use super::consts::SUPPORTED_LANGUAGES;

pub struct WiktionaryDownloader {
    pub language: String,
}

impl Downloader for WiktionaryDownloader {
    fn url(&self) -> String {
        let languages = SUPPORTED_LANGUAGES;
        let language = languages.get(self.language.as_str()).unwrap();

        format!(
            "https://kaikki.org/dictionary/{}/kaikki.org-dictionary-{}.jsonl",
            language, language
        )
    }

    fn new(language: Option<String>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        if let Some(lang) = language {
            if SUPPORTED_LANGUAGES.contains_key(lang.as_str()) {
                Ok(Self { language: lang })
            } else {
                anyhow::bail!("Unsupported language: {}", lang);
            }
        } else {
            anyhow::bail!("A language is required for the Wiktionary downloader");
        }
    }
}
