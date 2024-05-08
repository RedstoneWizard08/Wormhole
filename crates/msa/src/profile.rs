use anyhow::Result;
use reqwest::Client;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skin {
    pub id: String,
    pub state: String,
    pub url: String,
    pub variant: String,
    pub alias: Option<String>,
    pub texture_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cape {
    pub id: String,
    pub state: String,
    pub url: String,
    pub alias: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub skins: Vec<Skin>,
    pub capes: Vec<Cape>,
}

pub async fn get_profile(access_token: impl AsRef<str>) -> Result<Profile> {
    Ok(Client::new()
        .get("https://api.minecraftservices.com/minecraft/profile")
        .bearer_auth(access_token.as_ref())
        .send()
        .await?
        .json()
        .await?)
}
