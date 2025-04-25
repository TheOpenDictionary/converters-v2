use std::io::Read;

use anyhow::Context;
use console::Term;
use flate2::read::GzDecoder;
use tar::Archive;

use crate::processors::traits::Extractor;

use super::schema::FreeDictEntry;

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

                println!("Contents: {}", contents);
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
