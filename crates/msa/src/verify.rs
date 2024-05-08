use anyhow::Result;
use reqwest::Client;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entitlement {
    pub name: String,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entitlements {
    pub items: Vec<Entitlement>,
    pub signature: String,
    pub key_id: String,
}

pub async fn has_ownership(access_token: impl AsRef<str>) -> Result<bool> {
    let res = Client::new()
        .get("https://api.minecraftservices.com/entitlements/mcstore")
        .bearer_auth(access_token.as_ref())
        .send()
        .await?
        .json::<Entitlements>()
        .await?;

    let product = res.items.iter().any(|v| v.name == "product_minecraft");
    let game = res.items.iter().any(|v| v.name == "game_minecraft");

    Ok(product && game)
}
