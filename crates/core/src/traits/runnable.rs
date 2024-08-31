use anyhow::Result;

#[async_trait]
pub trait Runnable {
    async fn run(&self) -> Result<()>;
}
