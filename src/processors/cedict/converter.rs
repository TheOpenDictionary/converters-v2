use std::collections::HashMap;

use console::Term;
use map_macro::hash_map;
use odict::{Definition, DefinitionType, Dictionary, Entry, Etymology, ID, PartOfSpeech, Sense};

use crate::{processors::traits::Converter, progress::STYLE_PROGRESS};

use super::schema::CEDictEntry;

pub struct CEDictConverter {}

impl Converter for CEDictConverter {
    type Entry = CEDictEntry;

    fn convert(&mut self, term: &Term, data: &Vec<CEDictEntry>) -> anyhow::Result<Dictionary> {
        term.write_line("🔄 Converting the dictionary...")?;

        let progress = indicatif::ProgressBar::new(data.len() as u64);
        progress.set_style(STYLE_PROGRESS.clone());

        let mut entries: HashMap<String, Entry> = hash_map! {};

        for cedict_entry in data {
            progress.inc(1);
            progress.set_message(cedict_entry.simplified.clone());

            let simplified = cedict_entry.simplified.clone();
            let traditional = cedict_entry.traditional.clone();
            let pronunciation = cedict_entry.pronunciation.clone();

            // Create forms for traditional characters if different from simplified
            let mut forms = vec![];
            // if traditional != simplified {
            //     forms.push(Form {
            //         term: traditional.into(),
            //         kind: Some("traditional".to_string()),
            //     });
            // }

            // Create definitions
            let definitions = cedict_entry
                .definitions
                .iter()
                .map(|def| {
                    DefinitionType::Definition(Definition {
                        id: None,
                        value: def.clone(),
                        examples: vec![],
                        notes: vec![],
                    })
                })
                .collect();

            // Create sense with noun part of speech (CEDict doesn't specify POS)
            let sense = Sense {
                pos: PartOfSpeech::n, // Default to noun as most entries are nouns
                definitions,
            };

            // Create etymology with pronunciation
            let ety = Etymology {
                id: None,
                pronunciation: Some(pronunciation),
                description: None,
                senses: hash_map! {
                    PartOfSpeech::n => sense,
                },
            };

            // Add entry
            let entry = Entry {
                etymologies: vec![ety],
                term: simplified.clone(),
                forms,
                lemma: None,
                see_also: None,
            };

            entries.insert(simplified, entry);
        }

        progress.finish_and_clear();

        term.clear_last_lines(1)?;
        term.write_line("✅ Conversion complete")?;

        Ok(Dictionary {
            id: ID::new(),
            name: Some("CC-CEDICT".to_string()),
            entries,
        })
    }

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}
