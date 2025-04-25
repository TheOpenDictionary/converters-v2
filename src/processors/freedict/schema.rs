use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Welcome {
    tei: Tei,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tei {
    tei_header: TeiHeader,
    text: Text,
    #[serde(rename = "@xmlns")]
    xmlns: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeiHeader {
    file_desc: FileDesc,
    encoding_desc: EncodingDesc,
    revision_desc: RevisionDesc,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncodingDesc {
    project_desc: ProjectDesc,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectDesc {
    p: PClass,
}

#[derive(Serialize, Deserialize)]
pub struct PClass {
    #[serde(rename = "ref")]
    p_ref: Ref,
    #[serde(rename = "@_text")]
    text: String,
}

#[derive(Serialize, Deserialize)]
pub struct Ref {
    #[serde(rename = "@target")]
    target: String,
    #[serde(rename = "@_text")]
    text: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDesc {
    title_stmt: TitleStmt,
    edition_stmt: EditionStmt,
    extent: String,
    publication_stmt: PublicationStmt,
    notes_stmt: NotesStmt,
    source_desc: SourceDesc,
}

#[derive(Serialize, Deserialize)]
pub struct EditionStmt {
    edition: String,
}

#[derive(Serialize, Deserialize)]
pub struct NotesStmt {
    note: Note,
}

#[derive(Serialize, Deserialize)]
pub struct Note {
    #[serde(rename = "@type")]
    note_type: String,
    #[serde(rename = "@_text")]
    text: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicationStmt {
    publisher: String,
    availability: Availability,
    pub_place: PubPlace,
}

#[derive(Serialize, Deserialize)]
pub struct Availability {
    p: Vec<PUnion>,
    #[serde(rename = "@status")]
    status: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PUnion {
    PClass(PClass),
    String(String),
}

#[derive(Serialize, Deserialize)]
pub struct PubPlace {
    #[serde(rename = "ref")]
    pub_place_ref: Ref,
}

#[derive(Serialize, Deserialize)]
pub struct SourceDesc {
    p: Vec<PClass>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TitleStmt {
    title: String,
    resp_stmt: RespStmt,
}

#[derive(Serialize, Deserialize)]
pub struct RespStmt {
    resp: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct RevisionDesc {
    change: Vec<Change>,
}

#[derive(Serialize, Deserialize)]
pub struct Change {
    name: String,
    list: Option<List>,
    #[serde(rename = "@n")]
    n: Option<String>,
    #[serde(rename = "@when")]
    when: Option<String>,
    #[serde(rename = "@_text")]
    text: Option<String>,
    #[serde(rename = "@who")]
    who: Option<String>,
    date: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct List {
    item: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Text {
    body: Body,
}

#[derive(Serialize, Deserialize)]
pub struct Body {
    entry: Vec<Entry>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    form: Form,
    sense: SenseUnion,
    gram_grp: Option<EntryGramGrp>,
}

#[derive(Serialize, Deserialize)]
pub struct Form {
    orth: String,
}

#[derive(Serialize, Deserialize)]
pub struct EntryGramGrp {
    pos: Pos,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Pos {
    F,
    M,
    #[serde(rename = "m;f")]
    MF,
    N,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SenseUnion {
    SenseElement(SenseElement),
    SenseElementArray(Vec<SenseElement>),
}

#[derive(Serialize, Deserialize)]
pub struct SenseElement {
    cit: CitUnion,
    #[serde(rename = "@n")]
    n: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum CitUnion {
    CitElement(CitElement),
    CitElementArray(Vec<CitElement>),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitElement {
    quote: String,
    #[serde(rename = "@type")]
    cit_type: Type,
    gram_grp: Option<CitGramGrp>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Trans,
}

#[derive(Serialize, Deserialize)]
pub struct CitGramGrp {
    #[serde(rename = "gen")]
    gender: Pos,
}

pub struct FreeDictEntry {
    pub term: String,
    pub pronunciation: Option<String>,
    pub definitions: Vec<String>,
    pub pos: Option<Pos>,
}
