#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use api::{
    plugin::Plugin,
    plugins::{Kerbal1Plugin, Kerbal2Plugin, MinecraftPlugin},
    tauri::TauriPlugin,
    TAURI_HANDLE,
};
use tauri::Wry;
use wormhole_gui::{cmds, ctx, events, invoker};

#[tokio::main]
pub async fn main() -> Result<()> {
    let db = init::boot(&None).await?;

    let (_, setup_events) = tauri_specta::ts::builder()
        .commands(cmds())
        .events(events::<Wry>())
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
