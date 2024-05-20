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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftAuthRequest {
    #[serde(rename = "identityToken")]
    pub identity_token: String,
}

impl MinecraftAuthRequest {
    pub fn new((token, hash): (impl AsRef<str>, impl AsRef<str>)) -> Self {
        Self {
            identity_token: format!("XBL3.0 x={};{}", hash.as_ref(), token.as_ref()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftAuthResponse {
    pub username: String,
    pub roles: Vec<String>,
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
}

pub async fn do_mc_auth(data: (impl AsRef<str>, impl AsRef<str>)) -> Result<String> {
    let req = MinecraftAuthRequest::new(data);
    let client = Client::new();

    let res = client
        .post("https://api.minecraftservices.com/authentication/login_with_xbox")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(serde_json::to_string(&req)?)
        .send()
        .await?
        .text()
        .await?;

    let res = serde_json::from_str::<MinecraftAuthResponse>(&res)?;

    Ok(res.access_token)
}
