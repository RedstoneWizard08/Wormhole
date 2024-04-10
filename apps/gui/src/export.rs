use anyhow::Result;
use const_format::formatcp;
use specta::ts::{BigIntExportBehavior, ExportConfiguration};
use tauri_specta::ts::export_with_cfg;

pub const BINDINGS_PATH: &str =
    formatcp!("{}/../../src/api/bindings.ts", env!("CARGO_MANIFEST_DIR"));

#[test]
pub fn export_types() -> Result<()> {
    export_with_cfg(
        crate::get_funcs().unwrap(),
        ExportConfiguration::default().bigint(BigIntExportBehavior::BigInt),
        BINDINGS_PATH,
    )?;

    Ok(())
}
