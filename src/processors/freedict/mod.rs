use super::Processor;

mod converter;
mod downloader;
mod extractor;
mod schema;

pub struct FreeDictProcessor {}

impl Processor for FreeDictProcessor {
    type Entry = schema::FreeDictEntry;
    type Downloader = downloader::FreeDictDownloader;
    type Extractor = extractor::FreeDictExtractor;
    type Converter = converter::FreeDictConverter;

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}