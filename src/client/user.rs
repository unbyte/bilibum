use serde::Deserialize;

use crate::client::{Client, ParseResponse};

#[derive(Debug, Deserialize)]
struct RawUserInfo {
    #[serde(rename = "uname")]
    username: Option<String>,
    #[serde(rename = "vipStatus")]
    #[serde(default)]
    vip: bool,
}

#[derive(Debug)]
pub enum UserInfo {
    Anonymous,
    User { username: String, vip: bool },
}

impl From<RawUserInfo> for UserInfo {
    fn from(s: RawUserInfo) -> Self {
        if let Some(username) = s.username {
            UserInfo::User {
                username,
                vip: s.vip,
            }
        } else {
            UserInfo::Anonymous
        }
    }
}

impl Client {
    pub async fn query_user_info(&self) -> anyhow::Result<UserInfo> {
        let req = self
            .0
            .get("https://api.bilibili.com/x/web-interface/nav")
            .build()?;

        let resp = RawUserInfo::parse_response(self.0.execute(req).await?).await?;

        Result::from(resp).map(Into::into)
    }
}
