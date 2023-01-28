use crate::client::{Client, ParseResponse};
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize)]
pub struct VideoPost {
    #[serde(rename = "bvid")]
    pub id: String,
    pub title: String,
    #[serde(rename = "pic")]
    pub cover_url: String,
    #[serde(rename = "desc")]
    pub description: String,
    #[serde(rename = "ctime")]
    pub upload_time: u64,
    #[serde(rename = "pages")]
    pub videos: Vec<Video>,
}

#[derive(Debug, Deserialize)]
pub struct Video {
    #[serde(rename = "cid")]
    pub id: u64,
    #[serde(rename = "part")]
    pub title: String,
    pub duration: i64,
}

impl Display for Video {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}:{})",
            &self.title,
            self.duration / 60,
            self.duration % 60
        )
    }
}

impl Client {
    pub async fn get_video_post(&self, video_post_id: &str) -> anyhow::Result<VideoPost> {
        let req = self
            .0
            .get("https://api.bilibili.com/x/web-interface/view")
            .query(&[("bvid", video_post_id)])
            .build()?;

        let resp = VideoPost::parse_response(self.0.execute(req).await?).await?;

        Result::from(resp).map(Into::into)
    }
}
