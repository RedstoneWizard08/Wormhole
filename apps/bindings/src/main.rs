use anyhow::Result;
use const_format::formatcp;
use specta::ts::{formatter::prettier, BigIntExportBehavior, ExportConfig};
use tauri::Wry;
use tauri_specta::ts;

pub const BINDINGS_PATH: &str = formatcp!("{}/../../src/api/bindings/", env!("CARGO_MANIFEST_DIR"));
pub const BIG_INT: BigIntExportBehavior = BigIntExportBehavior::Number;

#[tokio::main]
pub async fn main() -> Result<()> {
    println!("Exporting app bindings...");

    ts::builder()
        .commands(wormhole_gui::cmds())
        .events(wormhole_gui::events::<Wry>())
        .path(format!("{}/app.ts", BINDINGS_PATH))
        .config(ExportConfig::default().formatter(prettier).bigint(BIG_INT))
        .export()?;

    Ok(())
}
