use anyhow::Result;
use tracing::level_filters::LevelFilter;

use crate::log::init_file_logger;

/// Initialize a logger and start the Tauri app.
pub async fn run() -> Result<()> {
    init_file_logger("./logs/app.log", LevelFilter::INFO)?;

    init::boot().await?;

    Ok(tauri::Builder::default().run(tauri::generate_context!())?)
}
