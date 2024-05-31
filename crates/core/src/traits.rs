use anyhow::Result;

/// An async version of the [`Default`] trait.
/// This allows implementors to provide a default method
/// that fetches the default value from an async source.
#[async_trait]
pub trait AsyncDefault {
    async fn default() -> Self;
}

#[async_trait]
impl<T: Default + Send + Sync> AsyncDefault for T {
    async fn default() -> Self {
        Default::default()
    }
}

/// A trait for objects that can be run.
/// This is most commonly used for CLI apps and their
/// (sub)commands.
#[async_trait]
pub trait Runnable {
    async fn run(&self) -> Result<()>;
}
