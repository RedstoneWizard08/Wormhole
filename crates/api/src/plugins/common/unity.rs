use std::path::PathBuf;

use anyhow::Result;
use data::instance::Instance;
use mcmeta::cmd::modded::ModLoader;
use query::source::Resolver;
use tokio::process::{Child, Command};

use crate::plugin::Plugin;

#[async_trait]
pub trait UnityPlugin: Send + Sync {
    fn new() -> Self
    where
        Self: Sized;

    fn id(&self) -> &'static str;
    fn game(&self) -> i32;
    fn icon(&self) -> String;
    fn banner(&self) -> String;
    fn display(&self) -> String;
    fn fallback(&self) -> Option<&'static str>;
    fn executable(&self) -> &'static str;
    fn find(&self) -> Option<PathBuf>;
    fn name(&self) -> &'static str;
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
