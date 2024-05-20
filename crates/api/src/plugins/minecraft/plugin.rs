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
use base64::{engine::general_purpose::STANDARD, Engine};
use data::instance::Instance;
use msa::state::MsaState;
use query::{curse::CurseForge, modrinth::Modrinth, source::Resolver};
use tokio::process::Child;

use crate::plugin::Plugin;

use super::manager::MinecraftManager;

pub const ICON_BYTES: &[u8] = include_bytes!("../../assets/minecraft/icon.svg");
pub const BANNER_BYTES: &[u8] = include_bytes!("../../assets/minecraft/banner.jpg");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MinecraftPlugin;

#[async_trait]
impl Plugin for MinecraftPlugin {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn id(&self) -> &'static str {
        "MC".into()
    }

    fn game(&self) -> i32 {
        432
    }

    fn icon(&self) -> String {
        format!("data:image/svg+xml;base64,{}", STANDARD.encode(ICON_BYTES))
    }

    fn banner(&self) -> String {
        format!("data:image/jpeg;base64,{}", STANDARD.encode(BANNER_BYTES))
    }

    fn display(&self) -> String {
        "Minecraft".into()
    }

    fn fallback(&self) -> Option<&'static str> {
        Some("mods")
    }

    async fn resolvers(&self) -> Vec<Box<dyn Resolver + Send + Sync>> {
        vec![
            Box::new(CurseForge::new().await),
            Box::new(Modrinth::new().await),
        ]
    }

    async fn launch(&self, instance: Instance) -> Result<Child> {
        let manager = MinecraftManager::load(instance.data_dir())?;

        manager.launch(&MsaState::get(), &instance).await
    }
}
