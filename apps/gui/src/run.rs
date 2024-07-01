use std::sync::Arc;

use anyhow::Result;
use commands::router::build_router;
use data::get_or_init_client;
use tracing::level_filters::LevelFilter;

use crate::log::init_file_logger;

/// Initialize a logger and start the Tauri app.
pub async fn run() -> Result<()> {
    init_file_logger("./logs/app.log", LevelFilter::INFO)?;

    init::boot().await?;
    let rspc = build_router();
    let db = get_or_init_client().await?;

    Ok(tauri::Builder::default()
        .plugin(rspc_tauri::plugin(Arc::new(rspc), move |_| db.clone()))
        .run(tauri::generate_context!())?)
}
