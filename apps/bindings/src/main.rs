use anyhow::Result;
use api::{
    plugin::Plugin,
    plugins::{Kerbal1Plugin, Kerbal2Plugin, MinecraftPlugin},
    tauri::{TauriPlugin, TauriPluginTrait},
};
use const_format::formatcp;
use specta::ts::{formatter::prettier, BigIntExportBehavior, ExportConfig};
use tauri::Wry;
use tauri_specta::ts;

pub const BINDINGS_PATH: &str = formatcp!("{}/../../src/api/bindings/", env!("CARGO_MANIFEST_DIR"));
pub const BIG_INT: BigIntExportBehavior = BigIntExportBehavior::Number;

#[tokio::main]
pub async fn main() -> Result<()> {
    println!("Exporting plugin bindings...");

    let plugins: Vec<Box<dyn TauriPluginTrait + Send + Sync + 'static>> = vec![
        Box::new(Kerbal1Plugin::new()),
        Box::new(Kerbal2Plugin::new()),
        Box::new(MinecraftPlugin::new()),
    ];

    for plugin in plugins {
        let it = TauriPlugin::<Wry>::new_boxed(plugin).unwrap();

        println!("Exporting plugin bindings: {}", it.name);

        ts::builder()
            .commands(it.cmds)
            .path(format!("{}/plugins/{}.ts", BINDINGS_PATH, it.name))
            .config(ExportConfig::default().formatter(prettier).bigint(BIG_INT))
            .export_for_plugin(it.name)?;
    }

    println!("Exporting app bindings...");

    ts::builder()
        .commands(wormhole_gui::cmds())
        .path(format!("{}/app.ts", BINDINGS_PATH))
        .config(ExportConfig::default().formatter(prettier).bigint(BIG_INT))
        .export()?;

    Ok(())
}
