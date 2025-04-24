use std::path::PathBuf;

use cedict::{CEDictArgs, CEDictConverter, CEDictDownloader, CEDictExtractor};
use clap::{Parser, Subcommand, command};
use console::Term;
use traits::{Converter, Downloader, Extractor};
use utils::save_dictionary;
use wiktionary::{WiktionaryArgs, WiktionaryConverter, WiktionaryDownloader, WiktionaryExtractor};

mod cedict;
mod progress;
mod traits;
mod utils;
mod wiktionary;

#[derive(Debug, Parser)]
#[command(name = "odict-convert")]
#[command(about = "Convert other dictionary formats to .odict files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, help = "Path to save the output dictionary file")]
    output: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Wiktionary(WiktionaryArgs),
    CEDict(CEDictArgs),
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let (command_name, language) = match &args.command {
        Commands::Wiktionary(wiktionary_args) => ("wiktionary", wiktionary_args.language.clone()),
        Commands::CEDict(_) => ("cedict", "zho-eng".to_string()),
    };

    let downloader: Box<dyn Downloader> = match args.command {
        Commands::Wiktionary(args) => Box::new(WiktionaryDownloader::new(args.language)),
        Commands::CEDict(_) => Box::new(CEDictDownloader::new()),
    };

    let extractor: Box<dyn Extractor> = match args.command {
        Commands::Wiktionary(_) => Box::new(WiktionaryExtractor::new()),
        Commands::CEDict(_) => Box::new(CEDictExtractor::new()),
    };

    let mut converter: Box<dyn Converter<Entry = _>> = match args.command {
        Commands::Wiktionary(_) => Box::new(WiktionaryConverter::new()),
        Commands::CEDict(_) => Box::new(CEDictConverter::new()),
    };

    let term = Term::stdout();
    let text = downloader.download(&term).await.unwrap();
    let parsed = extractor.extract(&term, &text).unwrap();
    let dictionary = converter.convert(&term, &parsed).unwrap();

    let output_path: PathBuf = match &args.output {
        Some(path) => path.clone().into(),
        None => format!("out/{}/{}.odict", command_name, language).into(),
    };

    save_dictionary(term, &dictionary, &output_path).unwrap();
}
