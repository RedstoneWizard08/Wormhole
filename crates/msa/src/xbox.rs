// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use anyhow::Result;
use reqwest::Client;

pub const XBOX_AUTH_METHOD: &str = "RPS";
pub const XBOX_SITE_NAME: &str = "user.auth.xboxlive.com";
pub const XBOX_RELYING_PARTY: &str = "http://auth.xboxlive.com";
pub const XBOX_TOKEN_TYPE: &str = "JWT";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XboxAuthRequest {
    #[serde(rename = "Properties")]
    pub properties: XboxAuthProperties,

    #[serde(rename = "RelyingParty")]
    pub relying_party: String,

    #[serde(rename = "TokenType")]
    pub token_type: String,
}

impl XboxAuthRequest {
    pub fn new(token: impl AsRef<str>) -> Self {
        Self {
            properties: XboxAuthProperties::new(token),
            relying_party: XBOX_RELYING_PARTY.into(),
            token_type: XBOX_TOKEN_TYPE.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XboxAuthProperties {
    #[serde(rename = "AuthMethod")]
    pub auth_method: String,

    #[serde(rename = "SiteName")]
    pub site_name: String,

    #[serde(rename = "RpsTicket")]
    pub rps_ticket: String,
}

impl XboxAuthProperties {
    pub fn new(token: impl AsRef<str>) -> Self {
        Self {
            auth_method: XBOX_AUTH_METHOD.into(),
            site_name: XBOX_SITE_NAME.into(),
            rps_ticket: format!("d={}", token.as_ref()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XboxAuthResponse {
    #[serde(rename = "IssueInstant")]
    pub issue_instant: String,

    #[serde(rename = "NotAfter")]
    pub not_after: String,

    #[serde(rename = "Token")]
    pub token: String,

    #[serde(rename = "DisplayClaims")]
    pub display_claims: DisplayClaims,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayClaims {
    #[serde(rename = "xui")]
    pub xui: Vec<DisplayClaim>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayClaim {
    pub uhs: String,
}

/// Takes in the token from Microsoft auth.
pub async fn do_xbox_auth(token: impl AsRef<str>) -> Result<(String, String)> {
    let req = XboxAuthRequest::new(token);
    let client = Client::new();

    let res = client
        .post("https://user.auth.xboxlive.com/user/authenticate")
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
