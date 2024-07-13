use anyhow::Result;
use api::seed::seed_plugins;
use data::get_or_init_client;
use msa::state::MsaState;
use plugins::register_defaults;
use whcore::manager::CoreManager;

pub static mut INIT: bool = false;

pub async fn boot() -> Result<()> {
    unsafe {
        if INIT {
            return Err(anyhow::anyhow!("Already initialized"));
        }

        INIT = true;
    }

    CoreManager::get().init();
    register_defaults().await;
    seed_plugins(get_or_init_client().await?).await?;

    tokio::spawn(async move {
        MsaState::init().await.unwrap();
    })
    .await?;

    Ok(())
}
