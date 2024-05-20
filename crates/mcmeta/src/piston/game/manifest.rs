// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use super::{
    args::GameArgs, assets::AssetIndexRef, download::GameDownloads, java::JavaRef,
    library::LibraryRef, logging::LoggingConfigs,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameManifest {
    pub arguments: Option<GameArgs>,
    pub asset_index: AssetIndexRef,
    pub assets: String,
    pub compliance_level: i32,
    pub downloads: GameDownloads,
    pub id: String,
    pub java_version: JavaRef,
    pub libraries: Vec<LibraryRef>,
    pub logging: Option<LoggingConfigs>,
    pub main_class: String,
    pub minimum_launcher_version: i32,
    pub release_time: String,
    pub time: String,
    pub minecraft_args: Option<String>,

    #[serde(rename = "type")]
    pub kind: String,
}

impl GameManifest {
    pub fn game_args(&self, feats: &Vec<String>) -> Vec<String> {
        let mut out = self
            .arguments
            .as_ref()
            .map(|v| v.get_game_args(feats))
            .unwrap_or_default();

        if let Some(args) = &self.minecraft_args {
            out.append(&mut args.split(" ").map(|v| v.to_string()).collect::<Vec<_>>());
        }

        out
    }

    pub fn java_args(&self, feats: &Vec<String>) -> Vec<String> {
        let mut args = self
            .arguments
            .as_ref()
            .map(|v| v.get_jvm_args(feats))
            .unwrap_or_default();

        if let Some(config) = &self.logging {
            if let Some(client) = &config.client {
                args.push(client.argument.clone());
            }
        }

        args
    }
}
