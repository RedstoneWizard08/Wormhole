use async_trait::async_trait;

#[async_trait]
pub trait AsyncDefault {
    async fn default() -> Self;
}

#[async_trait]
impl<T: Default> AsyncDefault for T {
    async fn default() -> Self {
        Default::default()
    }
}
