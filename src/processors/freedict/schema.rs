use serde::{Deserialize, Serialize};

// Final processed entry that will be converted to ODICT
#[derive(Debug)]
pub struct FreeDictEntry {
    pub term: String,
    pub pronunciation: Option<String>,
    pub definitions: Vec<String>,
}

// XML deserialization structures below

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "TEI")]
pub struct TeiDocument {
    #[serde(rename = "text")]
    pub text: Text,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Text {
    #[serde(rename = "body")]
    pub body: Body,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Body {
    #[serde(rename = "entry")]
    pub entries: Vec<Entry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    #[serde(rename = "form")]
    pub form: Form,
    #[serde(rename = "sense")]
    pub senses: Vec<Sense>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Form {
    #[serde(rename = "orth")]
    pub orth: Orth,
    #[serde(rename = "pron")]
    pub pron: Option<Pron>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Orth {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pron {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sense {
    #[serde(rename = "cit")]
    pub citations: Vec<Citation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Citation {
    #[serde(rename = "quote")]
    pub quote: Quote,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Quote {
    #[serde(rename = "$value")]
    pub value: String,
}

// FreeDict API response structures
#[derive(Debug, Deserialize, Serialize)]
pub struct FreeDictApiEntry {
    pub name: String,
    pub releases: Vec<FreeDictRelease>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FreeDictRelease {
    pub platform: String,
    #[serde(rename = "URL")]
    pub url: String,
}