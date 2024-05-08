use msa::state::MsaState;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LaunchOptions {
    /// The player's username.
    /// Ex: "Notch"
    pub player_name: String,

    /// The player's UUID.
    /// Ex: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
    pub uuid: String,

    /// The player's access token.
    /// This is received through MSA (auth).
    pub access_token: String,

    /// The player's XUID (Xbox User ID).
    pub xuid: String,

    /// The launcher name.
    /// Ex: "Wormhole"
    pub launcher_name: String,

    /// The launcher version.
    /// Ex: "2.0.0"
    pub launcher_version: String,

    /// The memory to launch with.
    /// Ex: 4096 (MB) for 4 GB
    pub memory: u64,
}

impl LaunchOptions {
    pub fn new(auth: &MsaState, memory: u64) -> Self {
        Self {
            launcher_name: "Wormhole".into(),
            launcher_version: env!("CARGO_PKG_VERSION").into(),
            memory,
            access_token: auth.access_token.clone(),
            player_name: auth.player_name.clone(),
            uuid: auth.uuid.clone(),
            xuid: auth.xuid.clone(),
        }
    }
}
