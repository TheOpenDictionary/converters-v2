use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CEDictEntry {
    /// Traditional Chinese characters
    pub traditional: String,
    /// Simplified Chinese characters
    pub simplified: String,
    /// Pronunciation in pinyin format
    pub pronunciation: String,
    /// List of definitions
    pub definitions: Vec<String>,
}
