use clap::Args;

mod converter;
mod downloader;
mod extractor;
mod schema;

pub use converter::*;
pub use downloader::*;
pub use extractor::*;

#[derive(Debug, Args)]
pub struct CEDictArgs {}