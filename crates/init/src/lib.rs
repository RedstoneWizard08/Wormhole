use anyhow::Result;
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

    tokio::spawn(async move {
        MsaState::init().await.unwrap();
    })
    .await?;

    Ok(())
}
