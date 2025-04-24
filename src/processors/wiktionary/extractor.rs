use console::Term;
use rayon::prelude::*;

use crate::{processors::traits::Extractor, progress::STYLE_PROGRESS};

use super::schema::WiktionaryEntry;

pub struct WiktionaryExtractor {}

impl Extractor for WiktionaryExtractor {
    type Entry = WiktionaryEntry;

    fn extract(&self, term: &Term, data: &Vec<u8>) -> anyhow::Result<Vec<WiktionaryEntry>> {
        term.write_line("🔍 Extracting the dictionary...")?;

        let text = String::from_utf8_lossy(data);

        let progress = indicatif::ProgressBar::new(text.lines().count() as u64);

        progress.set_style(STYLE_PROGRESS.clone());

        let result: Result<Vec<_>, _> = text
            .lines()
            .enumerate()
            .par_bridge()
            .map(|(i, l)| {
                progress.inc(1);
                serde_json::from_str(l)
                    .map_err(|e| anyhow::anyhow!("Failed to parse line {}: {} - {}", i + 1, e, l))
            })
            .collect();

        progress.finish_and_clear();

        term.clear_last_lines(1)?;
        term.write_line("✅ Extraction complete")?;

        Ok(result?)
    }

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}
