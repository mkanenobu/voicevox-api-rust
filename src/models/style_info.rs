/*
 * VOICEVOX Engine
 *
 * VOICEVOXの音声合成エンジンです。
 *
 * The version of the OpenAPI document: 0.14.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// StyleInfo : スタイルの追加情報



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StyleInfo {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "portrait", skip_serializing_if = "Option::is_none")]
    pub portrait: Option<String>,
    #[serde(rename = "voice_samples")]
    pub voice_samples: Vec<String>,
}

impl StyleInfo {
    /// スタイルの追加情報
    pub fn new(id: i32, icon: String, voice_samples: Vec<String>) -> StyleInfo {
        StyleInfo {
            id,
            icon,
            portrait: None,
            voice_samples,
        }
    }
}


