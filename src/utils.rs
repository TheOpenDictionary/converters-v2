use std::path::{Path, PathBuf};
use std::time::Duration;

use console::Term;
use odict::DictionaryWriter;

pub fn save_dictionary(
    term: Term,
    dictionary: &odict::Dictionary,
    output_path: &PathBuf,
) -> anyhow::Result<()> {
    if let Some(parent) = Path::new(&output_path).parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    let writer = DictionaryWriter::new();
    let spinner = indicatif::ProgressBar::new_spinner();

    if let Some(parent) = Path::new(&output_path).parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_message("Writing the dictionary to file (this might take a while)...");

    writer.write_to_path(&dictionary, &output_path).unwrap();

    spinner.finish_and_clear();

    term.clear_last_lines(1).unwrap();
    term.write_line(&format!(
        "✅ Dictionary written to {}",
        output_path.display()
    ))
    .unwrap();

    Ok(())
}
