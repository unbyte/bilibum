use crate::client::{unescape_unicode, Client, ParseResponse};
use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
struct RawQrCode {
    pub url: String,
    #[serde(rename = "qrcode_key")]
    pub key: String,
}

#[derive(Debug)]
pub struct QrCode {
    pub url: String,
    pub key: String,
}

impl From<RawQrCode> for QrCode {
    fn from(r: RawQrCode) -> Self {
        Self {
            url: match unescape_unicode(&r.url) {
                Cow::Owned(u) => u,
                _ => r.url,
            },
            key: r.key,
        }
    }
}

impl Client {
    pub async fn generate_qrcode(&self) -> anyhow::Result<QrCode> {
        let req = self
            .0
            .get("https://passport.bilibili.com/x/passport-login/web/qrcode/generate")
            .build()?;

        let resp = RawQrCode::parse_response(self.0.execute(req).await?).await?;

        Result::from(resp).map(Into::into)
    }
}
