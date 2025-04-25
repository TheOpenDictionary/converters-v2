use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FreeDictEntry {
    text: Text,
    #[serde(rename = "@xmlns")]
    xmlns: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EncodingDesc {
    project_desc: ProjectDesc,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectDesc {
    p: PClass,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PClass {
    #[serde(rename = "ref")]
    p_ref: Ref,
    #[serde(rename = "@_text")]
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ref {
    #[serde(rename = "@target")]
    target: String,
    #[serde(rename = "@_text")]
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    #[serde(rename = "@type")]
    note_type: String,
    #[serde(rename = "@_text")]
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct List {
    item: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Text {
    body: Body,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Body {
    entry: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    form: Form,
    sense: SenseUnion,
    gram_grp: Option<EntryGramGrp>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Form {
    orth: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntryGramGrp {
    pos: Pos,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Pos {
    F,
    M,
    #[serde(rename = "m;f")]
    MF,
    N,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SenseUnion {
    SenseElement(SenseElement),
    SenseElementArray(Vec<SenseElement>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SenseElement {
    #[serde(rename = "@n")]
    n: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Trans,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CitGramGrp {
    #[serde(rename = "gen")]
    gender: Pos,
}
