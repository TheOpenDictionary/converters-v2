use super::Processor;

mod converter;
mod downloader;
mod extractor;
mod schema;

pub struct CEDictProcessor {}

impl Processor for CEDictProcessor {
    type Entry = schema::CEDictEntry;
    type Downloader = downloader::CEDictDownloader;
    type Extractor = extractor::CEDictExtractor;
    type Converter = converter::CEDictConverter;

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}
