use async_trait::async_trait;

use crate::processors::traits::Downloader;

pub struct CEDictDownloader {}

impl CEDictDownloader {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait(?Send)]
impl Downloader for CEDictDownloader {
    async fn url(&self) -> anyhow::Result<String> {
        Ok(
            "https://www.mdbg.net/chinese/export/cedict/cedict_1_0_ts_utf-8_mdbg.txt.gz"
                .to_string(),
        )
    }

    fn new(language: Option<String>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        if language.is_some() {
            anyhow::bail!("CEDict downloader does not support language selection");
        }
        Ok(Self::new())
    }
}
