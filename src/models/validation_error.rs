/*
 * VOICEVOX Engine
 *
 * VOICEVOXの音声合成エンジンです。
 *
 * The version of the OpenAPI document: 0.14.4
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValidationError {
    #[serde(rename = "loc")]
    pub loc: Vec<String>,
    #[serde(rename = "msg")]
    pub msg: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl ValidationError {
    pub fn new(loc: Vec<String>, msg: String, r#type: String) -> ValidationError {
        ValidationError { loc, msg, r#type }
    }
}
