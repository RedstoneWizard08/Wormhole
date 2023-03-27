use async_trait::async_trait;
use std::path::PathBuf;

#[async_trait]
pub trait ModInstaller {
    fn new(install_path: PathBuf) -> Self;

    async fn install(&self, id: i32, instance_id: i32);
}
