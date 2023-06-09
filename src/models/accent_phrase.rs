/*
 * VOICEVOX Engine
 *
 * VOICEVOXの音声合成エンジンです。
 *
 * The version of the OpenAPI document: 0.14.4
 *
 * Generated by: https://openapi-generator.tech
 */

/// AccentPhrase : アクセント句ごとの情報

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccentPhrase {
    #[serde(rename = "moras")]
    pub moras: Vec<crate::models::Mora>,
    #[serde(rename = "accent")]
    pub accent: i32,
    #[serde(rename = "pause_mora", skip_serializing_if = "Option::is_none")]
    pub pause_mora: Option<Box<crate::models::Mora>>,
    #[serde(rename = "is_interrogative", skip_serializing_if = "Option::is_none")]
    pub is_interrogative: Option<bool>,
}

impl AccentPhrase {
    /// アクセント句ごとの情報
    pub fn new(moras: Vec<crate::models::Mora>, accent: i32) -> AccentPhrase {
        AccentPhrase {
            moras,
            accent,
            pause_mora: None,
            is_interrogative: None,
        }
    }
}
