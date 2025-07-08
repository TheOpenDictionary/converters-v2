use crate::processors::traits::Downloader;

pub struct CEDictDownloader {}

impl CEDictDownloader {
    pub fn new() -> Self {
        Self {}
    }
}

impl Downloader for CEDictDownloader {
    fn url(&self) -> String {
        "https://www.mdbg.net/chinese/export/cedict/cedict_1_0_ts_utf-8_mdbg.txt.gz".to_string()
    }

    fn new(_language: &Option<String>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        // CEDict is always Chinese, so we accept any language parameter but ignore it
        Ok(Self::new())
    }
}
