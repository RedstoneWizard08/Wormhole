use anyhow::Result;
use commands::router::build_router;
use data::get_or_init_client;
use tracing::level_filters::LevelFilter;

use crate::log::init_file_logger;

/// Initialize a logger and start the Tauri app.
pub async fn run() -> Result<()> {
    init_file_logger("./logs/app.log", LevelFilter::INFO)?;

    init::boot().await?;

    let db = get_or_init_client().await?;

    Ok(tauri::Builder::default()
        .plugin(build_router().tauri(db))
        .run(tauri::generate_context!())?)
}
