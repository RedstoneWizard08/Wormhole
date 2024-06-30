use crate::IntoAsync;

#[async_trait]
impl<N: Send + Sync, P: IntoAsync<N> + Send + Sync> IntoAsync<Vec<N>> for Vec<P> {
    async fn into_async(self) -> Vec<N> {
        let mut vec = Vec::new();

        for item in self {
            vec.push(item.into_async().await);
        }

        vec
    }
}
