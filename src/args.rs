use clap::{Args, arg};

use crate::processors::wiktionary::SUPPORTED_LANGUAGES;

#[derive(Debug, Args)]
pub struct WiktionaryArgs {
    #[arg(value_parser = SUPPORTED_LANGUAGES.keys().collect::<Vec<_>>())]
    pub language: String,
}
