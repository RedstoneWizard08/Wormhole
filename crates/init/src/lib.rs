use anyhow::Result;
use msa::state::MsaState;

pub static mut INIT: bool = false;

pub async fn boot() -> Result<()> {
    unsafe {
        if INIT {
            return Err(anyhow::anyhow!("Already initialized"));
        }

        INIT = true;
    }

    data::get_or_init_client().await?;

    tokio::spawn(async move {
        MsaState::init().await.unwrap();
    })
    .await?;

    Ok(())
}
