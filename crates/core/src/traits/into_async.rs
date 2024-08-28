#[async_trait]
pub trait IntoAsync<T> {
    async fn into_async(self) -> T;
}
