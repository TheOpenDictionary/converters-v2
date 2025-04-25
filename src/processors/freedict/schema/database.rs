// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use pub generated_module::Welcome;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let pub model: Welcome = pub serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};

pub type Welcome = Vec<WelcomeElement>;

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeElement {
    pub date: Option<String>,
    pub edition: Option<String>,
    pub headwords: Option<String>,
    pub maintainer_name: Option<MaintainerName>,
    pub name: Option<String>,
    pub releases: Option<Vec<Release>>,
    #[serde(rename = "sourceURL")]
    pub source_url: Option<String>,
    pub status: Option<Status>,
    pub maintainer_email: Option<String>,
    pub software: Option<Software>,
}

#[derive(Serialize, Debug, Deserialize)]
pub enum MaintainerName {
    #[serde(rename = "Beata Wójtowicz")]
    BeataWjtowicz,
    #[serde(rename = "Denis Arnaud")]
    DenisArnaud,
    #[serde(rename = "Dennis N.")]
    DennisN,
    #[serde(rename = "Einhard Leichtfuß")]
    EinhardLeichtfu,
    #[serde(rename = "Erdal Ronahi")]
    ErdalRonahi,
    #[serde(rename = "Fred Ulisses Maranhão")]
    FredUlissesMaranho,
    #[serde(rename = "FreeDict - no maintainer assigned")]
    FreeDictNoMaintainerAssigned,
    #[serde(rename = "Joe Hansen")]
    JoeHansen,
    #[serde(rename = "Karl Bartel")]
    KarlBartel,
    #[serde(rename = "Kevin Donnelly")]
    KevinDonnelly,
    #[serde(rename = "Michael Bunk")]
    MichaelBunk,
    #[serde(rename = "Piotr Bański")]
    PiotrBaski,
    #[serde(rename = "Raul Fernandes")]
    RaulFernandes,
    #[serde(rename = "Sebastian Humenda")]
    SebastianHumenda,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Release {
    #[serde(rename = "URL")]
    pub url: String,
    pub checksum: String,
    pub date: String,
    pub platform: Platform,
    pub size: String,
    pub version: String,
}

#[derive(Serialize, PartialEq, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    Dictd,
    Slob,
    Src,
    Stardict,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Software {
    pub tools: Tools,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Tools {
    #[serde(rename = "URL")]
    pub url: String,
    pub checksum: String,
    pub date: String,
    pub version: String,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Big,
    #[serde(rename = "big enough")]
    BigEnough,
    #[serde(rename = "Big enough to be useful")]
    BigEnoughToBeUseful,
    #[serde(rename = "growth stalled (Nov 05)")]
    GrowthStalledNov05,
    #[serde(rename = "low quality")]
    LowQuality,
    Small,
    Stable,
    #[serde(rename = "big enough to be useful")]
    StatusBigEnoughToBeUseful,
    #[serde(rename = "Too small")]
    StatusTooSmall,
    #[serde(rename = "too small")]
    TooSmall,
    #[serde(rename = "Too small to be useful")]
    TooSmallToBeUseful,
    Unstable,
}
