use crate::client::{unescape_unicode, Client, ParseResponse};
use serde::Deserialize;
use std::borrow::Cow;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
struct RawVideoStream {
    pub dash: RawDash,
}

#[derive(Debug, Deserialize)]
struct RawDash {
    duration: u64,
    audio: Vec<RawAudio>,
    flac: Option<RawAudio>,
}

#[derive(Debug, Deserialize)]
struct RawAudio {
    #[serde(rename = "id")]
    quality: u32,
    base_url: String,
    mime_type: String,
}

impl RawAudio {
    fn url(&self) -> Cow<str> {
        unescape_unicode(&self.base_url)
    }

    fn quality(&self) -> Option<AudioQuality> {
        match self.quality {
            30216 => Some(AudioQuality::_64K),
            30232 => Some(AudioQuality::_132K),
            30280 => Some(AudioQuality::_192K),
            30250 => Some(AudioQuality::Dolby),
            30251 => Some(AudioQuality::HiRes),
            _ => None,
        }
    }

    fn codec(&self) -> Option<AudioCodecType> {
        match self.mime_type.as_str() {
            "audio/mp4" => Some(AudioCodecType::AAC),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum AudioQuality {
    _64K,
    _132K,
    _192K,
    Dolby,
    HiRes,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AudioCodecType {
    AAC,
}

#[derive(Debug)]
pub struct Audio {
    pub url: String,
    pub duration: u64,
    pub quality: AudioQuality,
    pub codec: AudioCodecType,
}

impl Client {
    pub async fn get_audio_streams(
        &self,
        video_post_id: &str,
        video_id: u64,
    ) -> anyhow::Result<Vec<Audio>> {
        let req = self
            .0
            .get("https://api.bilibili.com/x/player/playurl")
            .query(&[
                ("bvid", video_post_id),
                ("cid", video_id.to_string().as_str()),
                // TODO: use fnval=272 to get dolby audio
                ("fnval", "16"),
            ])
            .build()?;

        let rv = Result::from(RawVideoStream::parse_response(self.0.execute(req).await?).await?)?;

        Ok(rv
            .dash
            .audio
            .into_iter()
            .chain(rv.dash.flac.into_iter())
            .filter_map(|ra| {
                let duration = rv.dash.duration;
                let quality = ra.quality()?;
                let codec = ra.codec()?;
                let url = match ra.url() {
                    Cow::Owned(u) => u,
                    _ => ra.base_url,
                };
                Some(Audio {
                    url,
                    duration,
                    quality,
                    codec,
                })
            })
            .collect())
    }
}
