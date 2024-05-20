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
use data::{
    instance::Instance,
    source::{SourceMapping, Sources},
};
use query::source::Resolver;
use tokio::process::Child;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Type)]
pub struct PluginInfo {
    pub id: &'static str,
    pub game: i32,
    pub display_name: String,
    pub icon_url: String,
    pub banner_url: String,
    pub fallback_dir: Option<&'static str>,
    pub resolvers: Vec<SourceMapping>,
}

unsafe impl Send for PluginInfo {}
unsafe impl Sync for PluginInfo {}

#[async_trait]
pub trait Plugin: Send + Sync {
    /// Create a new instance.
    fn new() -> Self
    where
        Self: Sized;

    /// Get the plugin's identifier.
    fn id(&self) -> &'static str;

    /// Get the game ID.
    fn game(&self) -> i32;

    /// Get a query resolver.
    async fn resolvers(&self) -> Vec<Box<dyn Resolver + Send + Sync>>;

    /// Get the display name.
    fn display(&self) -> String;

    /// Get the icon.
    fn icon(&self) -> String;

    /// Get the banner.
    fn banner(&self) -> String;

    /// Get the fallback mod install directory,
    /// relative to the game directory.
    /// If a mod fails all built-in conditions
    /// (Minecraft & BepInEx-specific built in
    /// at the time of writing), it will just
    /// extract all included files to this
    /// directory. This defaults to `BepInEx/plugins`.
    fn fallback(&self) -> Option<&'static str>;

    /// Get a source based on its ID.
    async fn get_source(&self, source: i32) -> Option<Box<dyn Resolver + Send + Sync>> {
        for src in self.resolvers().await {
            if src.source().id.unwrap() == source {
                return Some(src);
            }
        }

        None
    }

    async fn as_info(&self) -> PluginInfo {
        PluginInfo {
            id: self.id(),
            game: self.game(),
            banner_url: self.banner(),
            display_name: self.display(),
            icon_url: self.icon(),
            fallback_dir: self.fallback(),

            resolvers: self
                .resolvers()
                .await
                .iter()
                .map(|v| Sources::from(v.source()).into())
                .collect::<Vec<_>>(),
        }
    }

    async fn launch(&self, instance: Instance) -> Result<Child>;
}
