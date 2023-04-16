/*
 * VOICEVOX Engine
 *
 * VOICEVOXの音声合成エンジンです。
 *
 * The version of the OpenAPI document: 0.14.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AudioQuery : 音声合成用のクエリ



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AudioQuery {
    #[serde(rename = "accent_phrases")]
    pub accent_phrases: Vec<crate::models::AccentPhrase>,
    #[serde(rename = "speedScale")]
    pub speed_scale: f32,
    #[serde(rename = "pitchScale")]
    pub pitch_scale: f32,
    #[serde(rename = "intonationScale")]
    pub intonation_scale: f32,
    #[serde(rename = "volumeScale")]
    pub volume_scale: f32,
    #[serde(rename = "prePhonemeLength")]
    pub pre_phoneme_length: f32,
    #[serde(rename = "postPhonemeLength")]
    pub post_phoneme_length: f32,
    #[serde(rename = "outputSamplingRate")]
    pub output_sampling_rate: i32,
    #[serde(rename = "outputStereo")]
    pub output_stereo: bool,
    #[serde(rename = "kana", skip_serializing_if = "Option::is_none")]
    pub kana: Option<String>,
}

impl AudioQuery {
    /// 音声合成用のクエリ
    pub fn new(accent_phrases: Vec<crate::models::AccentPhrase>, speed_scale: f32, pitch_scale: f32, intonation_scale: f32, volume_scale: f32, pre_phoneme_length: f32, post_phoneme_length: f32, output_sampling_rate: i32, output_stereo: bool) -> AudioQuery {
        AudioQuery {
            accent_phrases,
            speed_scale,
            pitch_scale,
            intonation_scale,
            volume_scale,
            pre_phoneme_length,
            post_phoneme_length,
            output_sampling_rate,
            output_stereo,
            kana: None,
        }
    }
}


