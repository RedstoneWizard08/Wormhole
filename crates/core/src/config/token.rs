#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenConfig {
    /// The CurseForge auth token.
    /// If this is None, this defaults to Wormhole's
    /// CurseForge key.
    pub curseforge: Option<String>,

    /// A Modrinth auth token.
    /// This is optional, but will increase the rate
    /// limit if provided.
    pub modrinth: Option<String>,

    /// A GitHub auth token to increase the rate limit.
    /// This is optional. This is used for CKANDex.
    pub github: Option<String>,

    /// An auth token for Nexus.
    /// This backend is currently not implemented.
    pub nexus: Option<String>,
}
