use self::token::TokenConfig;

pub mod token;

/// Wormhole's configuration.
/// This will be used in the future.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub tokens: TokenConfig,
}
