//! The Tauri plugin API.

use anyhow::Result;
use data::{instance::Instance, mod_::DbMod, source::SourceMapping, Conn};

use mcmeta::cmd::modded::GetLoader;
use query::{
    mod_::{Mod, ModVersion},
    source::{Paginated, QueryOptions},
};

use serde_json::Value;
use specta::{
    functions::CollectFunctionsResult,
    ts::{BigIntExportBehavior, ExportConfig},
};
use std::sync::Arc;
use tauri_specta::ts;

use tauri::{
    generate_handler,
    plugin::{Plugin, Result as PResult},
    AppHandle, Invoke, Manager, Runtime,
};

use crate::{
    fn_alias,
    macros::{cmds, tauri_aliases},
    plugin::{Plugin as CPlugin, PluginInfo},
};

/// A Tauri command invoker.
pub type Invoker<R> = Box<dyn Fn(Invoke<R>) + Send + Sync + 'static>;

/// A Tauri plugin (struct form).
pub struct TauriPlugin<R: Runtime> {
    plugin: Arc<Box<dyn TauriPluginTrait + Send + Sync + 'static>>,

    /// The plugin's command invoker.
    pub handler: Arc<Invoker<R>>,

    /// The plugin's name.
    pub name: &'static str,

    /// The plugin's command handler and [`CollectFunctionsResult`].
    pub cmds: (CollectFunctionsResult, Invoker<R>),
}

impl<R: Runtime> TauriPlugin<R> {
    /// Create a new Tauri plugin based on its trait form.
    pub fn new<T: TauriPluginTrait + Send + Sync + 'static>(plugin: T) -> Result<Self> {
        Self::new_boxed(Box::new(plugin))
    }

    /// Create a new Tauri plugin based on the boxed version of its trait form.
    pub fn new_boxed(plugin: Box<dyn TauriPluginTrait + Send + Sync + 'static>) -> Result<Self> {
        tauri_aliases!();

        let handler = ts::builder()
            .config(ExportConfig::default().bigint(BigIntExportBehavior::Number))
            .commands::<Invoker<R>>(cmds!())
            .build_plugin_utils(plugin.id())?;

        Ok(Self {
            handler: Arc::new(handler),
            name: plugin.id(),
            plugin: Arc::new(plugin),
            cmds: cmds!(),
        })
    }
}

impl<R: Runtime> Plugin<R> for TauriPlugin<R> {
    fn name(&self) -> &'static str {
        self.name
    }

    fn initialize(&mut self, app: &AppHandle<R>, _config: Value) -> PResult<()> {
        app.manage(self.plugin.clone());

        Ok(())
    }

    fn extend_api(&mut self, invoke: Invoke<R>) {
        (self.handler)(invoke)
    }
}

/// A Tauri plugin (trait form).
///
/// This was originally used for my naive version of the web UI's
/// invoker system, but it's still used as the function proxy target
/// in the GUI.
#[async_trait]
pub trait TauriPluginTrait: CPlugin + Send + Sync {
    /// Get the plugin's identifier.
    async fn info(&self) -> Option<PluginInfo>;

    /// Get the plugin's search results.
    async fn search_mods(
        &self,
        resolver: SourceMapping,
        instance: Instance,
        query: Option<String>,
        opts: Option<QueryOptions>,
    ) -> Option<Paginated<Mod>>;

    /// Get a mod's information.
    async fn get_mod(&self, resolver: SourceMapping, id: String) -> Option<Mod>;

    /// Get mod's available versions.
    async fn get_mod_versions(
        &self,
        resolver: SourceMapping,
        instance: Instance,
        id: String,
    ) -> Option<Vec<ModVersion>>;

    /// Get a mod's version.
    async fn get_mod_version(
        &self,
        resolver: SourceMapping,
        instance: Instance,
        id: String,
        version: String,
    ) -> Option<ModVersion>;

    /// Get the latest available version of a mod.
    async fn get_latest_version(
        &self,
        resolver: SourceMapping,
        instance: Instance,
        id: String,
    ) -> Option<ModVersion>;

    /// Get the download URL for a mod.
    async fn get_download_url(
        &self,
        resolver: SourceMapping,
        instance: Instance,
        project: String,
        version: Option<String>,
    ) -> Option<String>;

    /// Install a mod.
    async fn install(
        &self,
        db: &mut Conn,
        item: Mod,
        version: Option<ModVersion>,
        instance: Instance,
    ) -> Option<()>;

    /// Uninstall a mod.
    async fn uninstall(&self, db: &mut Conn, item: DbMod, instance: Instance) -> Option<()>;

    /// Launch the game with the given instance.
    async fn launch_game(&self, instance: Instance) -> Option<()>;

    /// Get the plugin's sources (IDs).
    async fn sources(&self) -> Option<Vec<String>>;
}

#[async_trait]
impl<T: CPlugin + Send + Sync> TauriPluginTrait for T {
    async fn info(&self) -> Option<PluginInfo> {
        self.as_info().await
    }

    async fn search_mods(
        &self,
        resolver: SourceMapping,
        instance: Instance,
        query: Option<String>,
        opts: Option<QueryOptions>,
    ) -> Option<Paginated<Mod>> {
        let resolvers = self.resolvers().await?;
        let resolver = resolvers.iter().find(|v| v.source().mapping() == resolver);

        if let Some(resolver) = resolver {
            resolver
                .search(
                    &instance.loader().await.ok()?,
                    self.game().to_string(),
                    query.unwrap_or_default(),
                    opts,
                )
                .await
                .ok()
        } else {
            None
        }
    }

    async fn get_mod(&self, resolver: SourceMapping, id: String) -> Option<Mod> {
        let resolvers = self.resolvers().await?;
        let resolver = resolvers.iter().find(|v| v.source().mapping() == resolver);

        if let Some(resolver) = resolver {
            resolver.get_mod(id).await.ok()
        } else {
            None
        }
    }

    async fn get_mod_versions(
        &self,
        resolver: SourceMapping,
        instance: Instance,
        id: String,
    ) -> Option<Vec<ModVersion>> {
        let resolvers = self.resolvers().await?;
        let resolver = resolvers.iter().find(|v| v.source().mapping() == resolver);

        if let Some(resolver) = resolver {
            resolver
                .get_versions(&instance.loader().await.ok()?, id)
                .await
                .ok()
        } else {
            None
        }
    }

    async fn get_mod_version(
        &self,
        resolver: SourceMapping,
        instance: Instance,
        id: String,
        version: String,
    ) -> Option<ModVersion> {
        let resolvers = self.resolvers().await?;
        let resolver = resolvers.iter().find(|v| v.source().mapping() == resolver);

        if let Some(resolver) = resolver {
            resolver
                .get_version(&instance.loader().await.ok()?, id, version)
                .await
                .ok()
        } else {
            None
        }
    }

    async fn get_latest_version(
        &self,
        resolver: SourceMapping,
        instance: Instance,
        id: String,
    ) -> Option<ModVersion> {
        let resolvers = self.resolvers().await?;
        let resolver = resolvers.iter().find(|v| v.source().mapping() == resolver);

        if let Some(resolver) = resolver {
            resolver
                .get_latest_version(&instance.loader().await.ok()?, id)
                .await
                .ok()
        } else {
            None
        }
    }

    async fn get_download_url(
        &self,
        resolver: SourceMapping,
        instance: Instance,
        project: String,
        version: Option<String>,
    ) -> Option<String> {
        let resolvers = self.resolvers().await?;
        let resolver = resolvers.iter().find(|v| v.source().mapping() == resolver);

        if let Some(resolver) = resolver {
            resolver
                .get_download_url(&instance.loader().await.ok()?, project, version)
                .await
                .ok()
        } else {
            None
        }
    }

    async fn install(
        &self,
        db: &mut Conn,
        item: Mod,
        version: Option<ModVersion>,
        instance: Instance,
    ) -> Option<()> {
        self.install_mod(db, item, version, instance).await.ok()?;

        Some(())
    }

    async fn uninstall(&self, db: &mut Conn, item: DbMod, instance: Instance) -> Option<()> {
        self.uninstall_mod(db, item, instance).await.ok()?;

        Some(())
    }

    async fn launch_game(&self, instance: Instance) -> Option<()> {
        let _ = self.launch(instance).await.ok()?;

        Some(())
    }

    async fn sources(&self) -> Option<Vec<String>> {
        Some(
            self.resolvers()
                .await?
                .iter()
                .map(|v| v.source().mapping().as_str().to_string())
                .collect(),
        )
    }
}
