use std::collections::HashMap;

use console::Term;
use isolang::Language;

use crate::frequency::{ost::get_subtitle_frequencies, utils::map_to_ranks};

#[derive(Debug, Clone)]
pub struct DefaultFrequencyMap {
    map: HashMap<String, u32>, // word -> rank
}

#[async_trait::async_trait(?Send)]
impl<'a, 'b> super::traits::FrequencyMapImpl<'a, 'b> for DefaultFrequencyMap {
    async fn new(language: &'a str, term: &'b Term) -> anyhow::Result<Option<Self>> {
        if let Some(lang) = Language::from_639_3(language).and_then(|l| l.to_639_1()) {
            let frequencies = get_subtitle_frequencies(lang, term)
                .await
                .unwrap_or_default();

            return Ok(Some(Self {
                map: map_to_ranks(&frequencies),
            }));
        }

        term.write_line(&format!(
            "❌ Couldn't find frequency map for language \"{}\"",
            language
        ))?;

        Ok(None)
    }

    fn get_frequency(&self, word: &str) -> Option<u32> {
        // For non-Chinese or single characters, use exact match
        self.map.get(word).cloned()
    }
}
