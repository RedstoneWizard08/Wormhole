// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use anyhow::{anyhow, Result};

use crate::piston::manifest::get_manifest;

use super::{
    args::GameArgs, get_game_manifest, library::LibraryRef, logging::LoggingConfigs,
    manifest::GameManifest,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InheritedGameManifest {
    pub arguments: GameArgs,
    pub id: String,
    pub inherits_from: String,
    pub libraries: Vec<LibraryRef>,
    pub logging: Option<LoggingConfigs>,
    pub main_class: String,
    pub release_time: String,
    pub time: String,

    #[serde(rename = "type")]
    pub kind: String,
}

impl InheritedGameManifest {
    pub async fn resolve(&self) -> Result<GameManifest> {
        let version = get_manifest()
            .await?
            .find(&self.inherits_from)
            .ok_or(anyhow!("Cannot find the inherited version!"))?;

        let mut manifest = get_game_manifest(version.url).await?;
        let mut my_arguments = self.arguments.clone();

        if let Some(args) = manifest.arguments.as_mut() {
            args.game.append(&mut my_arguments.game);
            args.jvm.append(&mut my_arguments.jvm);
        }

        manifest.id = self.id.clone();
        manifest.libraries.append(&mut self.libraries.clone());

        if let Some(config) = &self.logging {
            manifest.logging = Some(config.clone());
        }

        manifest.main_class = self.main_class.clone();
        manifest.release_time = self.release_time.clone();
        manifest.time = self.time.clone();
        manifest.kind = self.kind.clone();

        Ok(manifest)
    }
}
