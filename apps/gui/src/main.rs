#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;

#[tokio::main]
pub async fn main() -> Result<()> {
    wormhole_gui::run::run().await
}
