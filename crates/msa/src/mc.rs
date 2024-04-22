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
