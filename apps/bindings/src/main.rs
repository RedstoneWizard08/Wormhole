// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

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
