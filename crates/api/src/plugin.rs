use std::{collections::HashMap, path::PathBuf, sync::Arc};

use anyhow::Result;
use data::{
    instance::Instance,
    mod_::DbMod,
    source::{SourceMapping, Sources},
    Conn,
};
use mcmeta::cmd::modded::ModLoader;
use query::{
    mod_::{Mod, ModVersion},
    source::Resolver,
};
use tokio::{process::Child, sync::Mutex};
use whcore::{dirs::Dirs, manager::CoreManager};

use crate::install::{install::install_mod, progress::tauri_progress, uninstall::uninstall_mod};

lazy_static! {
    pub static ref RESOLVERS: Arc<Mutex<HashMap<&'static str, Vec<Arc<Box<dyn Resolver + Send + Sync>>>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

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

// TODO: Install hook (for Minecraft loader installation)
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

    /// Get the mod loader.
    async fn loader(&self, instance: Instance) -> Result<ModLoader>;

    /// Get the plugin's query resolvers.
    async fn resolvers(&self) -> Option<Vec<Arc<Box<dyn Resolver + Send + Sync>>>> {
        self.bootstrap_resolvers().await.ok()?;

        RESOLVERS.lock().await.get(self.id()).cloned()
    }

    /// Bootstrap resolvers.
    async fn bootstrap_resolvers(&self) -> Result<()> {
        let mut lock = RESOLVERS.lock().await;

        if !lock.contains_key(self.id()) {
            let resolvers = self.create_resolvers().await;
            let mut resolvers_out = Vec::new();

            for mut item in resolvers {
                item.init().await?;

                resolvers_out.push(Arc::new(item));
            }

            lock.insert(self.id(), resolvers_out);
        }

        Ok(())
    }

    /// Create the plugin's query resolvers.
    async fn create_resolvers(&self) -> Vec<Box<dyn Resolver + Send + Sync>>;

    /// Install a mod loader to an instance.
    async fn install_loader(&self, instance: &Instance, loader: &ModLoader) -> Result<()>;

    /// Get the display name.
    fn display(&self) -> String;

    /// Get the icon.
    fn icon(&self) -> String;

    /// Get the banner.
    fn banner(&self) -> String;

    /// Find the game's install directory.
    fn find(&self) -> Option<PathBuf>;

    /// The game's name (for dirs)
    fn name(&self) -> &'static str;

    /// Get directories for the game.
    fn dirs(&self) -> Dirs {
        Dirs {
            cache: CoreManager::get().game_cache_dir(self.name()),
            data: CoreManager::get().game_data_dir(self.name()),
            temp: CoreManager::get().game_temp_dir(self.name()),
            root: CoreManager::get().dir(),
        }
    }

    /// Get the fallback mod install directory,
    /// relative to the game directory.
    /// If a mod fails all built-in conditions
    /// (Minecraft & BepInEx-specific built in
    /// at the time of writing), it will just
    /// extract all included files to this
    /// directory. This defaults to `BepInEx/plugins`.
    fn fallback(&self) -> Option<&'static str>;

    /// Get a source based on its ID.
    async fn get_source(&self, source: i32) -> Option<Arc<Box<dyn Resolver + Send + Sync>>> {
        for src in self.resolvers().await? {
            if src.source().id.unwrap() == source {
                return Some(src);
            }
        }

        None
    }

    async fn as_info(&self) -> Option<PluginInfo> {
        Some(PluginInfo {
            id: self.id(),
            game: self.game(),
            banner_url: self.banner(),
            display_name: self.display(),
            icon_url: self.icon(),
            fallback_dir: self.fallback(),

            resolvers: self
                .resolvers()
                .await?
                .iter()
                .map(|v| Sources::from(v.source()).into())
                .collect::<Vec<_>>(),
        })
    }

    async fn launch(&self, instance: Instance) -> Result<Child>;

    async fn install_mod(
        &self,
        db: &mut Conn,
        item: Mod,
        version: Option<ModVersion>,
        instance: Instance,
    ) -> Result<()>
    where
        Self: Sized,
    {
        install_mod(
            db,
            item,
            version,
            instance,
            Box::new(self),
            Some(Box::new(tauri_progress)),
        )
        .await?;

        Ok(())
    }

    async fn uninstall_mod(&self, db: &mut Conn, item: DbMod, instance: Instance) -> Result<()>
    where
        Self: Sized,
    {
        uninstall_mod(db, item, instance).await?;

        Ok(())
    }

    async fn install_instance(&self, _inst: &Instance) -> Result<()> {
        Ok(())
    }
}
