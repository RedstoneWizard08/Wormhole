//! The Unity game support module.

use std::path::PathBuf;

use anyhow::Result;
use data::instance::Instance;
use mcmeta::cmd::modded::ModLoader;
use query::source::Resolver;
use tokio::process::{Child, Command};

use crate::plugin::Plugin;

/// A plugin implementation for Unity games.
/// This will automatically implement the [`Plugin`] trait.
#[async_trait]
pub trait UnityPlugin: Send + Sync {
    /// Create a new instance of the plugin.
    fn new() -> Self
    where
        Self: Sized;

    /// The id of the plugin.

    fn id(&self) -> &'static str;

    /// The game of the plugin.
    fn game(&self) -> i32;

    /// The icon of the plugin.
    fn icon(&self) -> String;

    /// The banner of the plugin.
    fn banner(&self) -> String;

    /// The display name of the plugin.
    fn display(&self) -> String;

    /// The fallback install dir of the plugin.
    fn fallback(&self) -> Option<&'static str>;

    /// The executable for the plugin to run when
    /// launching the game.
    fn executable(&self) -> &'static str;

    /// The install dir of the plugin.
    fn find(&self) -> Option<PathBuf>;

    /// The name of the plugin.
    fn name(&self) -> &'static str;

    /// Create the resolvers for the plugin.
    async fn resolvers(&self) -> Vec<Box<dyn Resolver + Send + Sync>>;
}

#[async_trait]
impl<T: UnityPlugin> Plugin for T {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self::new()
    }

    fn id(&self) -> &'static str {
        self.id()
    }

    fn game(&self) -> i32 {
        self.game()
    }

    fn icon(&self) -> String {
        self.icon()
    }

    fn banner(&self) -> String {
        self.banner()
    }

    fn display(&self) -> String {
        self.display()
    }

    fn fallback(&self) -> Option<&'static str> {
        self.fallback()
    }

    fn find(&self) -> Option<PathBuf> {
        self.find()
    }

    fn name(&self) -> &'static str {
        self.name()
    }

    async fn create_resolvers(&self) -> Vec<Box<dyn Resolver + Send + Sync>> {
        self.resolvers().await
    }

    async fn launch(&self, instance: Instance) -> Result<Child> {
        Ok(Command::new(instance.install_dir().join(self.executable())).spawn()?)
    }

    async fn loader(&self, _instance: Instance) -> Result<ModLoader> {
        Ok(ModLoader::None)
    }

    async fn install_loader(&self, _instance: &Instance, _loader: &ModLoader) -> Result<()> {
        Ok(())
    }
}
