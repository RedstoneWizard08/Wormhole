use crate::{cmds, ctx, events, invoker, log::init_file_logger};
use anyhow::Result;
use api::{
    plugin::Plugin,
    plugins::{Kerbal1Plugin, Kerbal2Plugin, MinecraftPlugin},
    tauri::TauriPlugin,
    TAURI_HANDLE,
};
use specta::ts::{BigIntExportBehavior, ExportConfig};
use tauri::Wry;
use tracing::level_filters::LevelFilter;

/// Initialize a logger and start the Tauri app.
pub async fn run() -> Result<()> {
    init_file_logger("./logs/app.log", LevelFilter::INFO)?;

    let db = init::boot(&None).await?;

    let (_, setup_events) = tauri_specta::ts::builder()
        .commands(cmds())
        .events(events::<Wry>())
        .config(ExportConfig::default().bigint(BigIntExportBehavior::Number))
        .build()?;

    let app = tauri::Builder::default()
        .plugin(TauriPlugin::new(Kerbal1Plugin::new())?)
        .plugin(TauriPlugin::new(Kerbal2Plugin::new())?)
        .plugin(TauriPlugin::new(MinecraftPlugin::new())?)
        .manage(db)
        .invoke_handler(invoker())
        .setup(|app| {
            setup_events(app);

            Ok(())
        })
        .build(ctx())?;

    *TAURI_HANDLE.lock().await = Some(app.handle());

    app.run(|_handle, _event| {});

    Ok(())
}
