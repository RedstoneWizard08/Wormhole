use std::path::PathBuf;
use async_trait::async_trait;

#[async_trait]
pub trait ModInstaller {
    fn new(install_path: PathBuf) -> Self;

    async fn install(&self, id: i32, instance_id: i32);
}
