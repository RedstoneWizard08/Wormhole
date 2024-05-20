// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use std::{fs, env};

fn main() {
    let config_path = format!("{}/tauri.conf.json.in", env!("CARGO_MANIFEST_DIR"));
    let config_path_out = format!("{}/tauri.conf.json", env!("CARGO_MANIFEST_DIR"));
    let root = format!("{}/../..", env!("CARGO_MANIFEST_DIR"));
    let version = env!("CARGO_PKG_VERSION");

    let mut data = fs::read_to_string(config_path).unwrap();

    data = data.replace("<version>", version);
    data = data.replace("<root>", &root);

    fs::write(config_path_out, data).unwrap();
}
