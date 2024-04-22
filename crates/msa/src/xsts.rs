use anyhow::Result;
use reqwest::Client;

use crate::xbox::XboxAuthResponse;

pub const XSTS_RELYING_PARTY: &str = "rp://api.minecraftservices.com/";
pub const XSTS_TOKEN_TYPE: &str = "JWT";
pub const XSTS_SANDBOX_ID: &str = "RETAIL";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XstsTokenRequest {
    #[serde(rename = "Properties")]
    pub properties: XstsTokenProperties,

    #[serde(rename = "RelyingParty")]
    pub relying_party: String,

    #[serde(rename = "TokenType")]
    pub token_type: String,
}

impl XstsTokenRequest {
    pub fn new(token: impl AsRef<str>) -> Self {
        Self {
            properties: XstsTokenProperties::new(token),
            relying_party: XSTS_RELYING_PARTY.into(),
            token_type: XSTS_TOKEN_TYPE.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XstsTokenProperties {
    #[serde(rename = "SandboxId")]
    pub sandbox_id: String,

    #[serde(rename = "UserTokens")]
    pub user_tokens: Vec<String>,
}

impl XstsTokenProperties {
    pub fn new(token: impl AsRef<str>) -> Self {
        Self {
            sandbox_id: XSTS_SANDBOX_ID.into(),
            user_tokens: vec![token.as_ref().to_string()],
        }
    }
}

/// Takes in the token from Xbox auth.
pub async fn do_xsts_auth(token: impl AsRef<str>) -> Result<(String, String)> {
    let req = XstsTokenRequest::new(token);
    let client = Client::new();

    let res = client
        .post("https://xsts.auth.xboxlive.com/xsts/authorize")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(serde_json::to_string(&req)?)
        .send()
        .await?
        .text()
        .await?;

    let res = serde_json::from_str::<XboxAuthResponse>(&res)?;

    Ok((
        res.token,
        res.display_claims.xui.first().unwrap().uhs.clone(),
    ))
}
