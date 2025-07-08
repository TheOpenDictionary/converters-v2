mod hsk;

use std::collections::HashMap;

use console::Term;

use crate::frequency::{traits::FrequencyMapImpl, utils::map_to_ranks};

#[derive(Debug, Clone)]
pub struct ChineseFrequencyMap {
    map: HashMap<String, u32>,
}

const RARENESS_THRESHOLD: u32 = 5000;

#[async_trait::async_trait(?Send)]
impl<'a, 'b> FrequencyMapImpl<'a, 'b> for ChineseFrequencyMap {
    async fn new(_language: &'a str, term: &'b Term) -> anyhow::Result<Option<Self>> {
        let traditional = super::ost::get_subtitle_frequencies("zh_TW", term).await?;
        let simplified = super::ost::get_subtitle_frequencies("zh_CN", term).await?;

        let mut ranks = traditional
            .into_iter()
            .chain(simplified)
            .collect::<HashMap<String, u32>>();

        ranks = map_to_ranks(&ranks);

        match hsk::get_hsk_ranks(&term).await {
            Ok(hsk_map) => {
                for (word, hsk_rank) in hsk_map.iter() {
                    ranks.insert(word.clone(), *hsk_rank);
                }
            }
            Err(e) => {
                term.write_line(&format!(
                    "⚠️ Failed to load HSK data ({}), falling back to OpenSubtitles only",
                    e
                ))?;
            }
        }

        return Ok(Some(Self { map: ranks }));
    }

    /**
     * Retrieves the frequency of a word from the frequency map.
     * If the word is a single character, it returns its exact frequency.
     * For multi-character words, it checks for an exact match first,
     * and if not found, it calculates the average frequency based on its characters.
     */
    fn get_frequency(&self, word: &str) -> Option<u32> {
        if word.chars().count() == 1 {
            return self.map.get(word).cloned();
        }

        let exact_match = self.map.get(word).cloned();
        let char_based = self.get_character_frequency(word);

        return match (exact_match, char_based) {
            (Some(exact_rank), Some(char_rank)) => Some(if exact_rank > RARENESS_THRESHOLD {
                char_rank
            } else {
                exact_rank
            }),
            (Some(exact_rank), None) => Some(exact_rank),
            (None, Some(char_rank)) => Some(char_rank),
            (None, None) => None,
        };
    }
}

impl ChineseFrequencyMap {
    /**
     * Calculates the average frequency of characters in a word.
     * This is useful for multi-character words where the exact frequency may not be available.
     */
    fn get_character_frequency(&self, word: &str) -> Option<u32> {
        let ranks: Vec<u32> = word
            .chars()
            .filter_map(|char| self.map.get(&char.to_string()).copied())
            .collect();

        if ranks.is_empty() {
            return None;
        }

        Some(ranks.iter().sum::<u32>() / ranks.len() as u32)
    }
}
