use data::instance::Instance;
#[cfg(test)]
use mcmeta::cmd::modded::ModLoader;
use msa::state::MsaState;
use whcore::manager::CoreManager;

use crate::{plugins::minecraft::manager::MinecraftManager, test_util::library_download_callback};

#[tokio::test]
pub async fn test_it_works() -> anyhow::Result<()> {
    let inst = Instance {
        id: None,
        cache_dir: CoreManager::get()
            .game_cache_dir("minecraft")
            .to_str()
            .unwrap()
            .into(),
        install_dir: CoreManager::get()
            .game_data_dir("minecraft")
            .to_str()
            .unwrap()
            .into(),
        data_dir: CoreManager::get()
            .game_data_dir("minecraft")
            .join("instances")
            .join("test")
            .to_str()
            .unwrap()
            .into(),
        description: String::new(),
        name: "Test Instance".into(),
        created: 0,
        updated: 0,
        game_id: 432,
    };

    let mut mgr =
        MinecraftManager::load_or_create(inst.data_dir(), &ModLoader::vanilla_latest().await?)
            .await?;

    mgr.install_loader(
        &ModLoader::fabric_latest().await?,
        &Some(Box::new(library_download_callback)),
    )
    .await?;

    mgr.launch(&MsaState::init().await?, &inst)
        .await?
        .wait()
        .await?;

    Ok(())
}
