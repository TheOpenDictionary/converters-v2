use std::path::PathBuf;

use self::commands::Commands;
use clap::{Parser, command};
use console::Term;
use frequency::FrequencyMap;
use processors::{CEDictProcessor, Processor, WiktionaryProcessor};
use utils::save_dictionary;

mod args;
mod commands;
mod frequency;
mod processors;
mod progress;
mod utils;

#[derive(Debug, Parser)]
#[command(name = "odict-convert")]
#[command(about = "Convert other dictionary formats to .odict files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, help = "Path to save the output dictionary file")]
    output: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let term = Term::stdout();

    match &args.command {
        Commands::TestFrequency { language, word } => {
            term.write_line(&format!(
                "🔍 Testing frequency for '{}' in language '{}'",
                word, language
            ))
            .unwrap();

            match FrequencyMap::new(language, &term).await.unwrap() {
                Some(freq_map) => {
                    match freq_map.get_frequency(word) {
                        Some(rank) => {
                            term.write_line(&format!(
                                "✅ Word '{}' has frequency rank: {}",
                                word, rank
                            ))
                            .unwrap();

                            // Convert rank to approximate proficiency level
                            let level = match rank {
                                1..=1000 => "A1",
                                1001..=2000 => "A2",
                                2001..=3000 => "B1",
                                3001..=5000 => "B2",
                                5001..=8000 => "C1",
                                _ => "C2+",
                            };
                            term.write_line(&format!(
                                "📊 Approximate proficiency level: {}",
                                level
                            ))
                            .unwrap();
                        }
                        None => {
                            term.write_line(&format!(
                                "❌ Word '{}' not found in frequency data",
                                word
                            ))
                            .unwrap();
                        }
                    }
                }
                None => {
                    term.write_line(&format!(
                        "❌ No frequency data available for language '{}'",
                        language
                    ))
                    .unwrap();
                }
            }
            return;
        }
        _ => {}
    }

    let dictionary = match &args.command {
        Commands::Wiktionary(wiktionary_args) => WiktionaryProcessor::new()
            .unwrap()
            .process(&term, Some(wiktionary_args.language.clone()))
            .await
            .unwrap(),
        Commands::CEDict => CEDictProcessor::new()
            .unwrap()
            .process(&term, Some("cmn".to_string()))
            .await
            .unwrap(),
        Commands::TestFrequency { .. } => unreachable!(),
    };

    let (command_name, language) = match &args.command {
        Commands::Wiktionary(wiktionary_args) => ("wiktionary", wiktionary_args.language.clone()),
        Commands::CEDict => ("cedict", "zho-eng".to_string()),
        Commands::TestFrequency { .. } => unreachable!(),
    };

    let output_path: PathBuf = match &args.output {
        Some(path) => path.clone().into(),
        None => format!("out/{}/{}.odict", command_name, language).into(),
    };

    save_dictionary(term, &dictionary, &output_path).unwrap();
}
