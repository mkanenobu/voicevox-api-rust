/*
 * VOICEVOX Engine
 *
 * VOICEVOXの音声合成エンジンです。
 *
 * The version of the OpenAPI document: 0.14.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SpeakerStyle : スピーカーのスタイル情報



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SpeakerStyle {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "id")]
    pub id: i32,
}

impl SpeakerStyle {
    /// スピーカーのスタイル情報
    pub fn new(name: String, id: i32) -> SpeakerStyle {
        SpeakerStyle {
            name,
            id,
        }
    }
}


