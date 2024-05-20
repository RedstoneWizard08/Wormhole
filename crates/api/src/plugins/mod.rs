// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

pub mod ksp1;
pub mod ksp2;
pub mod minecraft;

pub use ksp1::Kerbal1Plugin;
pub use ksp2::Kerbal2Plugin;
pub use minecraft::MinecraftPlugin;

use crate::{plugin::Plugin, register::register_plugin, tauri::TauriPluginTrait};

pub fn default_plugins() -> Vec<Box<dyn TauriPluginTrait>> {
    vec![
        Box::new(Kerbal1Plugin::new()),
        Box::new(Kerbal2Plugin::new()),
        Box::new(MinecraftPlugin::new()),
    ]
}

pub fn register_defaults() {
    for plugin in default_plugins() {
        register_plugin(plugin);
    }
}
