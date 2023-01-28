use async_trait::async_trait;
use reqwest::cookie::Jar;
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::borrow::Cow;
use std::fmt::Debug;
use std::sync::Arc;

mod qrcode;
mod qrcode_status;
mod stream;
mod user;
mod video;

pub use qrcode::*;
pub use qrcode_status::*;
pub use stream::*;
pub use user::*;
pub use video::*;

#[derive(Debug)]
pub struct Client(reqwest::Client);

impl Client {
    const TOKEN_NAME: &'static str = "SESSDATA";

    pub fn new(token: Option<String>) -> Self {
        let jar = Arc::new(Jar::default());
        if let Some(t) = token {
            let url = Url::parse("bilibili.com").unwrap();
            let cookie = format!("{}={}", Self::TOKEN_NAME, t);
            jar.add_cookie_str(&cookie, &url)
        }
        Self(
            reqwest::ClientBuilder::new()
                .cookie_provider(jar)
                .build()
                .unwrap(),
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct BiliResponse<T: Debug> {
    pub code: u32,
    pub message: String,
    pub data: T,
}

impl<T: Debug> From<BiliResponse<T>> for anyhow::Result<T> {
    fn from(r: BiliResponse<T>) -> Self {
        if r.code == 0 {
            Ok(r.data)
        } else {
            Err(anyhow::anyhow!("error from bilibili: {}", r.message))
        }
    }
}

#[async_trait]
pub trait ParseResponse: Sized + DeserializeOwned + Debug {
    async fn parse_response(resp: reqwest::Response) -> reqwest::Result<BiliResponse<Self>> {
        resp.json().await
    }
}

impl<T: Sized + DeserializeOwned + Debug> ParseResponse for T {}

pub(in crate::client) fn unescape_unicode(origin: &str) -> Cow<str> {
    lazy_regex::regex_replace_all!(r#"\\u(\d{4})"#, origin, |_, num: &str| {
        std::char::from_u32(u32::from_str_radix(num, 16).unwrap())
            .unwrap()
            .to_string()
    })
}
