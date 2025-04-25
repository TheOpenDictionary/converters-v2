use clap::{Args, arg};

use crate::processors::wiktionary::SUPPORTED_LANGUAGES;

#[derive(Debug, Args)]
pub struct WiktionaryArgs {
    #[arg(value_parser = SUPPORTED_LANGUAGES.keys().collect::<Vec<_>>())]
    pub language: String,
}

#[derive(Debug, Args)]
pub struct FreeDictArgs {
    #[arg(help = "Optional language pair (e.g., 'eng-fra' for English-French)")]
    pub language_pair: Option<String>,
}
