use std::io::Read;

use anyhow::Context;
use console::Term;
use liblzma::read::XzDecoder;
use quick_xml::de::from_str;
use tar::Archive;

use crate::processors::traits::Extractor;

use super::schema::tei::FreeDictEntry;

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

        let decoder = XzDecoder::new(&data[..]);
        let mut archive = Archive::new(decoder);

        for file_result in archive.entries()? {
            let mut file = file_result.context("Failed to read tar entry")?;
            let path = file.path().context("Failed to get file path")?;
            let path_str = path.to_string_lossy();

            if path_str.ends_with(".tei") {
                term.write_line(&format!("📄 Processing TEI file: {}", path_str))?;

                // Read the TEI XML file
                let mut contents = String::new();

                file.read_to_string(&mut contents)?;

                let entries: Vec<Self::Entry> =
                    from_str(&contents).context("Failed to parse TEI XML")?;
                println!("Contents: {:?}", entries);
                return Ok(entries);
            }
        }

        anyhow::bail!("No TEI files found in the archive");
    }
}
