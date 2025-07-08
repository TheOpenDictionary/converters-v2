use async_trait::async_trait;
use console::Term;

#[async_trait(?Send)]
pub trait FrequencyMapImpl<'a, 'b>: std::fmt::Debug {
    async fn new(language: &'a str, term: &'b Term) -> anyhow::Result<Option<Self>>
    where
        Self: Sized;

    fn get_frequency(&self, word: &str) -> Option<u32>;
}
