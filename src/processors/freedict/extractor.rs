use std::io::Read;

use anyhow::Context;
use console::Term;
use liblzma::read::XzDecoder;
use quick_xml::de::from_str;
use tar::Archive;

use crate::processors::{freedict::schema::tei::TEI, traits::Extractor};

use super::schema::tei::Entry as FreeDictEntry;

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
                let mut contents = String::new();

                file.read_to_string(&mut contents)?;

                let entries: TEI = from_str(&contents).context("Failed to parse TEI XML")?;

                return Ok(entries.text.body.entries);
            }
        }

        anyhow::bail!("No TEI files found in the archive");
    }
}
