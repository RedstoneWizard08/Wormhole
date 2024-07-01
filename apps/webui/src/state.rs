//! The state module.

use crate::cli::Cli;
use anyhow::Result;

/// The state for the Web UI.
#[derive(Clone)]
pub struct AppState {
    /// The CLI options.
    pub options: Cli,
}

impl AppState {
    /// Create a new state object.
    pub async fn new(options: Cli) -> Result<Self> {
        Ok(Self { options })
    }
}
