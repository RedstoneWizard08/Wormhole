use anyhow::Result;
use data::source::SourceMapping;

use query::{
    mod_::{Mod, ModVersion},
    source::{Paginated, QueryOptions},
};

use serde_json::Value;
use specta::{functions::CollectFunctionsResult, ts::{BigIntExportBehavior, ExportConfig}};
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

pub type Invoker<R> = Box<dyn Fn(Invoke<R>) + Send + Sync + 'static>;

pub struct TauriPlugin<R: Runtime> {
    plugin: Arc<Box<dyn TauriPluginTrait + Send + Sync + 'static>>,

    pub handler: Arc<Invoker<R>>,
    pub name: &'static str,
    pub cmds: (CollectFunctionsResult, Invoker<R>),
}

impl<R: Runtime> TauriPlugin<R> {
    pub fn new<T: TauriPluginTrait + Send + Sync + 'static>(plugin: T) -> Result<Self> {
        Self::new_boxed(Box::new(plugin))
    }

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

#[async_trait]
pub trait TauriPluginTrait: CPlugin {
    async fn info(&self) -> Option<PluginInfo>;

    async fn search_mods(
        &self,
        resolver: SourceMapping,
        query: Option<String>,
        opts: Option<QueryOptions>,
    ) -> Option<Paginated<Mod>>;

    async fn get_mod(&self, resolver: SourceMapping, id: String) -> Option<Mod>;

    async fn get_mod_versions(
        &self,
        resolver: SourceMapping,
        id: String,
    ) -> Option<Vec<ModVersion>>;

    async fn get_mod_version(
        &self,
        resolver: SourceMapping,
        id: String,
        version: String,
    ) -> Option<ModVersion>;

    async fn get_download_url(
        &self,
        resolver: SourceMapping,
        project: String,
        version: Option<String>,
    ) -> Option<String>;
}

#[async_trait]
impl<T: CPlugin + Send + Sync> TauriPluginTrait for T {
    async fn info(&self) -> Option<PluginInfo> {
        Some(self.as_info().await)
    }

    async fn search_mods(
        &self,
        resolver: SourceMapping,
        query: Option<String>,
        opts: Option<QueryOptions>,
    ) -> Option<Paginated<Mod>> {
        let resolvers = self.resolvers().await;
        let resolver = resolvers.iter().find(|v| v.source().mapping() == resolver);

        if let Some(resolver) = resolver {
            resolver
                .search(self.game().to_string(), query.unwrap_or_default(), opts)
                .await
                .ok()
        } else {
            None
        }
    }

    async fn get_mod(&self, resolver: SourceMapping, id: String) -> Option<Mod> {
        let resolvers = self.resolvers().await;
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
        id: String,
    ) -> Option<Vec<ModVersion>> {
        let resolvers = self.resolvers().await;
        let resolver = resolvers.iter().find(|v| v.source().mapping() == resolver);

        if let Some(resolver) = resolver {
            resolver.get_versions(id).await.ok()
        } else {
            None
        }
    }

    async fn get_mod_version(
        &self,
        resolver: SourceMapping,
        id: String,
        version: String,
    ) -> Option<ModVersion> {
        let resolvers = self.resolvers().await;
        let resolver = resolvers.iter().find(|v| v.source().mapping() == resolver);

        if let Some(resolver) = resolver {
            resolver.get_version(id, version).await.ok()
        } else {
            None
        }
    }

    async fn get_download_url(
        &self,
        resolver: SourceMapping,
        project: String,
        version: Option<String>,
    ) -> Option<String> {
        let resolvers = self.resolvers().await;
        let resolver = resolvers.iter().find(|v| v.source().mapping() == resolver);

        if let Some(resolver) = resolver {
            resolver.get_download_url(project, version).await.ok()
        } else {
            None
        }
    }
}
