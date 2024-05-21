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
