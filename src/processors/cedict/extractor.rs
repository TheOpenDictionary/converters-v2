use console::Term;
use flate2::read::GzDecoder;
use rayon::prelude::*;
use regex::Regex;
use std::io::{Cursor, Read};

use crate::{processors::traits::Extractor, progress::STYLE_PROGRESS};

use super::schema::CEDictEntry;

pub struct CEDictExtractor {}

impl Extractor for CEDictExtractor {
    type Entry = CEDictEntry;

    fn extract(&self, term: &Term, data: &Vec<u8>) -> anyhow::Result<Vec<CEDictEntry>> {
        term.write_line("🔍 Extracting the dictionary...")?;

        // First, we need to decompress the gzipped data
        let mut decoder = GzDecoder::new(Cursor::new(data));
        let mut decompressed = String::new();
        decoder.read_to_string(&mut decompressed)?;

        let lines: Vec<_> = decompressed
            .lines()
            .filter(|line| !line.starts_with('#') && !line.is_empty())
            .collect();

        let progress = indicatif::ProgressBar::new(lines.len() as u64);
        progress.set_style(STYLE_PROGRESS.clone());

        let regex = Regex::new(r"(.*?)\s(.*?)\s\[(.*?)]\s/(.*)/").unwrap();

        let results: Vec<CEDictEntry> = lines
            .par_iter()
            .filter_map(|line| {
                progress.inc(1);

                if let Some(captures) = regex.captures(line) {
                    let traditional = captures.get(1)?.as_str().to_string();
                    let simplified = captures.get(2)?.as_str().to_string();
                    let pronunciation = captures.get(3)?.as_str().to_string();
                    let definitions_str = captures.get(4)?.as_str();

                    let definitions = definitions_str
                        .split('/')
                        .filter(|s| !s.is_empty())
                        .map(|s| s.to_string())
                        .collect();

                    Some(CEDictEntry {
                        traditional,
                        simplified,
                        pronunciation,
                        definitions,
                    })
                } else {
                    None
                }
            })
            .collect();

        progress.finish_and_clear();

        term.clear_last_lines(1)?;
        term.write_line("✅ Extraction complete")?;

        Ok(results)
    }

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}
