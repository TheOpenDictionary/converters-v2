use crate::traits::Downloader;

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
}
