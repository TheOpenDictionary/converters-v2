use map_macro::hash_map;
use odict::PartOfSpeech;
use std::{collections::HashMap, sync::LazyLock};

pub const SUPPORTED_LANGUAGES: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    hash_map! {
        "eng" => "English",
        "fra" => "French",
        "ger" => "German",
        "ita" => "Italian",
        "pol" => "Polish",
        "spa" => "Spanish",
        "swe" => "Swedish",
        "jpn" => "Japanese",
        "rus" => "Russian",
        "ara" => "Arabic",
        "cmn" => "Chinese"
    }
});

pub const POS_MAP: LazyLock<HashMap<&str, PartOfSpeech>> = LazyLock::new(|| {
    hash_map! {
      "abbrev" => PartOfSpeech::abv,
      "adv_phrase" => PartOfSpeech::phr_adv,
      "affix" => PartOfSpeech::aff,
      "article" => PartOfSpeech::art,
      "character" => PartOfSpeech::chr,
      "circumpos" => PartOfSpeech::un,  // TODO: fix this? ger
      "combining_form" => PartOfSpeech::un,  // TODO: fix this? eng
      "romanization" => PartOfSpeech::un,  // TODO: fix this? jap
      "contraction" => PartOfSpeech::contr,
      "circumfix" => PartOfSpeech::cf,
      "infix" => PartOfSpeech::inf,
      "interfix" => PartOfSpeech::intf,
      "noun" => PartOfSpeech::n,
      "particle" => PartOfSpeech::part,
      "phrase" => PartOfSpeech::phr,
      "prefix" => PartOfSpeech::pref,
      "prep_phrase" => PartOfSpeech::phr_prep,
      "proverb" => PartOfSpeech::prov,
      "punct" => PartOfSpeech::punc,
      "suffix" => PartOfSpeech::suff,
      "symbol" => PartOfSpeech::sym,
      "verb" => PartOfSpeech::v
    }
});
