use query::source::Resolver;

#[async_trait]
pub trait Plugin: Send + Sync {
    /// Create a new instance.
    fn new() -> Self
    where
        Self: Sized;

    /// Get the plugin's identifier.
    fn id(&self) -> String;

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

    /// Get the fallback mod install directory.
    /// If a mod fails all built-in conditions
    /// (Minecraft & BepInEx-specific built in
    /// at the time of writing), it will just
    /// extract all included files to this
    /// directory. This defaults to `BepInEx/plugins`.
    fn fallback(&self) -> Option<&str>;

    /// Get a source based on its ID.
    async fn get_source(&self, source: i32) -> Option<Box<dyn Resolver + Send + Sync>> {
        for src in self.resolvers().await {
            if src.source().id.unwrap() == source {
                return Some(src);
            }
        }

        None
    }
}
