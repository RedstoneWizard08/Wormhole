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
