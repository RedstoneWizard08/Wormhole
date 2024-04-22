#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsaTokenResponse {
    pub token_type: String,
    pub scope: String,
    pub expires_in: i32,
    pub ext_expires_in: i32,
    pub access_token: String,
}
