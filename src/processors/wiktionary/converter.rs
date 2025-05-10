use std::collections::HashMap;

use crate::{frequency::FrequencyMap, processors::traits::Converter, progress::STYLE_PROGRESS};
use console::Term;
use map_macro::hash_map;
use odict::{
    Definition, DefinitionType, Dictionary, Entry, EntryRef, Etymology, Form, Group, ID,
    PartOfSpeech, Sense,
};

use super::{consts::POS_MAP, schema::WiktionaryEntry};

pub struct WiktionaryConverter {
    missing_pos: Vec<String>,
}

impl WiktionaryConverter {
    fn resolve_pos<'a>(&mut self, entry: &WiktionaryEntry) -> PartOfSpeech {
        let mut pos = PartOfSpeech::un;

        if let Some(resolved_pos) = entry
            .pos
            .as_ref()
            .and_then(|p| POS_MAP.get(p.as_str()).cloned())
        {
            pos = resolved_pos;
        } else if let Some(pos_value) = &entry.pos {
            if !self.missing_pos.contains(pos_value) {
                self.missing_pos.push(pos_value.clone());
            }
        }

        pos
    }
}

impl Converter for WiktionaryConverter {
    type Entry = WiktionaryEntry;

    fn convert(
        &mut self,
        term: &Term,
        frequency_map: &Option<FrequencyMap>,
        data: &Vec<WiktionaryEntry>,
    ) -> anyhow::Result<Dictionary> {
        term.write_line("🔄 Converting the dictionary...")?;

        self.missing_pos = vec![];

        let progress = indicatif::ProgressBar::new(data.len() as u64);

        progress.set_style(STYLE_PROGRESS.clone());

        let mut entries: HashMap<String, Entry> = hash_map! {};

        for entry in data {
            let pos = self.resolve_pos(&entry);
            let term = entry.word.to_owned();
            let see_also = entry.redirects.as_ref().map(|r| r[0].to_owned());
            let etymology_text = entry.etymology_text.to_owned();

            let mut definitions: Vec<DefinitionType> = vec![];
            let mut group_map: HashMap<String, usize> = hash_map! {};

            let mut lemma: Option<EntryRef> = None;
            let mut tags: Vec<String> = vec![];

            for sense in &entry.senses {
                tags.extend(sense.tags.iter().cloned());

                if sense.form_of.len() > 0 {
                    if let Some(fo) = &sense.form_of.get(0) {
                        lemma = Some(EntryRef::from(fo.word.to_owned()));
                    }
                }

                // Glosses with 2 senses typically have subdefinitions
                if sense.glosses.len() == 2 {
                    let parent = sense.glosses[0].to_owned();
                    let child = sense.glosses[1].to_owned();
                    let definition = Definition {
                        id: None,
                        value: child.to_owned(),
                        examples: vec![],
                        notes: vec![],
                    };

                    if let Some(&idx) = group_map.get(&parent) {
                        if let DefinitionType::Group(group) = &mut definitions[idx] {
                            group.definitions.push(definition);
                        }
                    } else {
                        let group = DefinitionType::Group(Group {
                            id: None,
                            description: parent.to_owned(),
                            definitions: vec![Definition {
                                id: None,
                                value: child.to_owned(),
                                examples: vec![],
                                notes: vec![],
                            }],
                        });
                        definitions.push(group);
                        group_map.insert(parent, definitions.len() - 1);
                    }
                } else if let Some(gloss) = sense.glosses.get(0) {
                    let definition = Definition {
                        id: None,
                        value: gloss.to_owned(),
                        examples: vec![],
                        notes: vec![],
                    };
                    definitions.push(DefinitionType::Definition(definition));
                }
            }

            let etymology_number = entry.etymology_number.unwrap_or(1);

            let forms = entry
                .forms
                .iter()
                .map(|f| Form {
                    kind: None,
                    term: EntryRef::from(f.form.to_owned()),
                    tags: f.tags.to_owned(),
                })
                .collect();

            let sense = Sense {
                pos: pos.to_owned(),
                lemma: lemma.to_owned(),
                tags,
                translations: vec![],
                forms,
                definitions: definitions.to_owned(),
            };

            if let Some(ety) = entries
                .get_mut(&term)
                .and_then(|e| e.etymologies.get_mut(etymology_number as usize - 1))
            {
                if let Some(sense) = ety.senses.get_mut(&pos) {
                    sense.definitions.append(&mut definitions);
                } else {
                    ety.senses.insert(pos, sense);
                }
            } else {
                let ety = Etymology {
                    id: None,
                    pronunciations: vec![],
                    description: etymology_text.to_owned(),
                    senses: hash_map! {
                        pos.to_owned() => sense,
                    },
                };

                if let Some(entry) = entries.get_mut(&term) {
                    entry.etymologies.push(ety);
                } else {
                    let entry = Entry {
                        etymologies: vec![ety],
                        term: term.to_owned(),
                        media: vec![],
                        see_also: see_also.map(|s| EntryRef::from(s)),
                    };

                    entries.insert(term.to_owned(), entry);
                }
            }

            progress.set_message(term);
            progress.inc(1);
        }

        progress.finish_and_clear();

        term.clear_last_lines(1)?;
        term.write_line("✅ Conversion complete")?;

        Ok(Dictionary {
            id: ID::new(),
            name: None,
            entries,
        })
    }

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            missing_pos: vec![],
        })
    }
}
