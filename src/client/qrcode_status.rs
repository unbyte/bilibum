use derive_more::Display;
use serde::Deserialize;

use crate::client::{Client, ParseResponse};

#[derive(Debug, Deserialize)]
struct RawQrCodeStatus {
    code: u32,
}

#[derive(Debug, Clone, Eq, PartialEq, Display)]
pub enum QrCodeStatus {
    #[display(fmt = "succeed with token ({})", token)]
    Succeed { token: String },
    #[display(fmt = "expired")]
    Expired,
    #[display(fmt = "scanned but not confirmed")]
    Scanned,
    #[display(fmt = "prepared but not scanned")]
    Prepared,
    #[display(fmt = "unknown")]
    Unknown,
}

impl Client {
    pub async fn query_qrcode_status(&self, key: &str) -> anyhow::Result<QrCodeStatus> {
        let req = self
            .0
            .get("https://passport.bilibili.com/x/passport-login/web/qrcode/poll")
            .query(&[("qrcode_key", key)])
            .build()?;

        let resp = self.0.execute(req).await?;
        let token = resp
            .cookies()
            .find(|c| c.name() == Self::TOKEN_NAME)
            .map(|c| c.value().to_string());

        let resp = RawQrCodeStatus::parse_response(resp).await?;

        let result = match resp.data.code {
            0 => QrCodeStatus::Succeed {
                token: token.expect("token is missing"),
            },
            86038 => QrCodeStatus::Expired,
            86090 => QrCodeStatus::Scanned,
            86101 => QrCodeStatus::Prepared,
            _ => QrCodeStatus::Unknown,
        };

        Ok(result)
    }
}
