use console::Term;
use indicatif::ProgressBar;
use odict::{
    Dictionary, 
    elements::{definition::Definition, entry::Entry, etymology::Etymology, usage::Usage}
};

use crate::{processors::traits::Converter, progress::STYLE_PROGRESS};

use super::schema::FreeDictEntry;

pub struct FreeDictConverter {}

impl Converter for FreeDictConverter {
    type Entry = FreeDictEntry;

    fn new() -> anyhow::Result<Self> where Self: Sized {
        Ok(Self {})
    }

    fn convert(&mut self, term: &Term, data: &Vec<Self::Entry>) -> anyhow::Result<Dictionary> {
        term.write_line("🔄 Converting to ODICT format...")?;
        
        let mut dictionary = Dictionary::new("FreeDict");
        let pb = ProgressBar::new(data.len() as u64);
        
        pb.set_style(STYLE_PROGRESS.clone());
        
        for entry_data in data {
            // Create definitions from each definition string
            let definitions: Vec<Definition> = entry_data
                .definitions
                .iter()
                .map(|def_str| Definition::new(def_str.to_string()))
                .collect();
            
            // Create a usage with the definitions
            let usage = Usage::new(None, definitions);
            
            // Create an etymology with the usage
            let mut etymology = Etymology::new(None);
            etymology.usages.push(usage);
            
            // Create the entry with the term, pronunciation, and etymology
            let mut odict_entry = Entry::new(entry_data.term.clone());
            
            if let Some(pron) = &entry_data.pronunciation {
                odict_entry.set_pronunciation(pron.clone());
            }
            
            odict_entry.etymologies.push(etymology);
            
            // Add the entry to the dictionary
            dictionary.add_entry(odict_entry)?;
            
            pb.inc(1);
        }
        
        pb.finish_and_clear();
        term.write_line(&format!("✅ Converted {} entries to ODICT format", data.len()))?;
        
        Ok(dictionary)
    }
}