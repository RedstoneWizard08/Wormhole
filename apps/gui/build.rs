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
use const_format::formatcp;
use std::fs;

pub const CONFIG_PATH: &str = formatcp!("{}/tauri.conf.json.in", env!("CARGO_MANIFEST_DIR"));
pub const CONFIG_PATH_OUT: &str = formatcp!("{}/tauri.conf.json", env!("CARGO_MANIFEST_DIR"));

pub const ROOT: &str = formatcp!("{}/../..", env!("CARGO_MANIFEST_DIR"));
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<()> {
    let mut data = fs::read_to_string(CONFIG_PATH)?;

    data = data.replace("<version>", VERSION);
    data = data.replace("<root>", &ROOT.replace("\\", "/")); // windows sucks

    fs::write(CONFIG_PATH_OUT, data)?;
    tauri_build::build();

    println!("cargo:rerun-if-env-changed=CARGO_PKG_VERSION");
    println!("cargo:rerun-if-env-changed=CARGO_MANIFEST_DIR");

    Ok(())
}
