/*
 * VOICEVOX Engine
 *
 * VOICEVOXの音声合成エンジンです。
 *
 * The version of the OpenAPI document: 0.14.4
 *
 * Generated by: https://openapi-generator.tech
 */

/// WordTypes :      fastapiでword_type引数を検証する時に使用するクラス     

///      fastapiでword_type引数を検証する時に使用するクラス     
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WordTypes {
    #[serde(rename = "PROPER_NOUN")]
    ProperNoun,
    #[serde(rename = "COMMON_NOUN")]
    CommonNoun,
    #[serde(rename = "VERB")]
    Verb,
    #[serde(rename = "ADJECTIVE")]
    Adjective,
    #[serde(rename = "SUFFIX")]
    Suffix,
}

impl ToString for WordTypes {
    fn to_string(&self) -> String {
        match self {
            Self::ProperNoun => String::from("PROPER_NOUN"),
            Self::CommonNoun => String::from("COMMON_NOUN"),
            Self::Verb => String::from("VERB"),
            Self::Adjective => String::from("ADJECTIVE"),
            Self::Suffix => String::from("SUFFIX"),
        }
    }
}

impl Default for WordTypes {
    fn default() -> WordTypes {
        Self::ProperNoun
    }
}
