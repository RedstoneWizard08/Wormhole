use crate::{cmds, ctx, events, invoker, log::init_file_logger};
use anyhow::Result;
use api::TAURI_HANDLE;
use specta::ts::{BigIntExportBehavior, ExportConfig};
use tauri::Wry;
use tracing::level_filters::LevelFilter;
use plugins::ext::PluginRegister;

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
        .add_plugins()?
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
