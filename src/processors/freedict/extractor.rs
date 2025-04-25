use std::io::Read;

use anyhow::Context;
use console::Term;
use flate2::read::GzDecoder;
use quickxml::{Reader, de::from_str, events::Event};
use tar::Archive;

use crate::processors::traits::Extractor;

use super::schema::{FreeDictEntry, TeiDocument};

pub struct FreeDictExtractor {}

impl Extractor for FreeDictExtractor {
    type Entry = FreeDictEntry;

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }

    fn extract(&self, term: &Term, data: &Vec<u8>) -> anyhow::Result<Vec<Self::Entry>> {
        term.write_line("🔍 Extracting dictionary data from archive...")?;

        // First attempt to decompress as gzip (most FreeDict archives are gzipped)
        let decoder = GzDecoder::new(&data[..]);
        let mut archive = Archive::new(decoder);

        let mut entries = Vec::new();
        let mut found_tei = false;

        // Go through each file in the tar archive
        for file_result in archive.entries()? {
            let mut file = file_result.context("Failed to read tar entry")?;
            let path = file.path().context("Failed to get file path")?;
            let path_str = path.to_string_lossy();

            // Look for TEI files
            if path_str.ends_with(".tei") {
                found_tei = true;
                term.write_line(&format!("📄 Processing TEI file: {}", path_str))?;

                // Read the TEI XML file
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;

                // Use quickxml to deserialize the XML into our schema structures
                match from_str::<TeiDocument>(&contents) {
                    Ok(document) => {
                        // Process entries from the deserialized document
                        for entry in document.text.body.entries {
                            let term = entry.form.orth.value.trim().to_string();

                            if term.is_empty() {
                                continue;
                            }

                            // Get pronunciation if available
                            let pronunciation = entry.form.pron.map(|p| p.value);

                            // Collect definitions from senses
                            let mut definitions = Vec::new();
                            for sense in entry.senses {
                                for citation in sense.citations {
                                    let definition =
                                        citation.quote.value.trim().replace('\n', "; ");
                                    if !definition.is_empty() {
                                        definitions.push(definition);
                                    }
                                }
                            }

                            if !definitions.is_empty() {
                                entries.push(FreeDictEntry {
                                    term,
                                    pronunciation,
                                    definitions,
                                });
                            }
                        }
                    }
                    Err(e) => {
                        // If the XML structure doesn't match our schema exactly, log the error
                        // but try to continue with a fallback parser
                        term.write_line(&format!(
                            "⚠️ Error deserializing XML: {}, trying fallback parser...",
                            e
                        ))?;

                        // Use the fallback parser
                        let fallback_entries = self
                            .parse_xml_fallback(&contents)
                            .context("Failed to parse XML with fallback parser")?;

                        entries.extend(fallback_entries);
                    }
                }

                break; // Process only the first TEI file found
            }
        }

        if !found_tei {
            anyhow::bail!("No TEI files found in the archive");
        }

        term.write_line(&format!("✅ Extracted {} entries", entries.len()))?;
        Ok(entries)
    }
}

impl FreeDictExtractor {
    // Fallback parser using manual event processing for more flexible XML handling
    fn parse_xml_fallback(&self, xml_content: &str) -> anyhow::Result<Vec<FreeDictEntry>> {
        let mut reader = Reader::from_str(xml_content);
        reader.trim_text(true);

        let mut entries = Vec::new();
        let mut buf = Vec::new();

        // Current entry being processed
        let mut current_term = String::new();
        let mut current_pronunciation = None;
        let mut current_definitions = Vec::new();
        let mut in_entry = false;
        let mut in_orth = false;
        let mut in_pron = false;
        let mut in_quote = false;

        // Process XML events
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name().as_ref() {
                        b"entry" => {
                            in_entry = true;
                            // Reset for new entry
                            current_term = String::new();
                            current_pronunciation = None;
                            current_definitions = Vec::new();
                        }
                        b"orth" => in_orth = true,
                        b"pron" => in_pron = true,
                        b"quote" => in_quote = true,
                        _ => {}
                    }
                }
                Ok(Event::Text(e)) => {
                    if in_orth {
                        current_term = e.unescape().unwrap_or_default().trim().to_string();
                    } else if in_pron {
                        current_pronunciation =
                            Some(e.unescape().unwrap_or_default().trim().to_string());
                    } else if in_quote {
                        let def = e.unescape().unwrap_or_default().trim().replace('\n', "; ");
                        if !def.is_empty() {
                            current_definitions.push(def);
                        }
                    }
                }
                Ok(Event::End(ref e)) => {
                    match e.name().as_ref() {
                        b"entry" => {
                            in_entry = false;
                            // Save the entry if it has a term and at least one definition
                            if !current_term.is_empty() && !current_definitions.is_empty() {
                                entries.push(FreeDictEntry {
                                    term: current_term.clone(),
                                    pronunciation: current_pronunciation.clone(),
                                    definitions: current_definitions.clone(),
                                });
                            }
                        }
                        b"orth" => in_orth = false,
                        b"pron" => in_pron = false,
                        b"quote" => in_quote = false,
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(anyhow::anyhow!("Error parsing XML: {}", e)),
                _ => {}
            }
            buf.clear();
        }

        Ok(entries)
    }
}
