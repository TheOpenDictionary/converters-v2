use super::Processor;

mod consts;
mod converter;
mod downloader;
mod extractor;
mod schema;

pub use consts::SUPPORTED_LANGUAGES;

pub struct WiktionaryProcessor {}

impl Processor for WiktionaryProcessor {
    type Entry = schema::WiktionaryEntry;
    type Downloader = downloader::WiktionaryDownloader;
    type Extractor = extractor::WiktionaryExtractor;
    type Converter = converter::WiktionaryConverter;

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}
