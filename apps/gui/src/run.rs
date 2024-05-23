use crate::{cmds, ctx, events, invoker};
use anyhow::Result;
use api::{
    plugin::Plugin,
    plugins::{Kerbal1Plugin, Kerbal2Plugin, MinecraftPlugin},
    tauri::TauriPlugin,
    TAURI_HANDLE,
};
use specta::ts::{BigIntExportBehavior, ExportConfig};
use tauri::Wry;

pub async fn run() -> Result<()> {
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
