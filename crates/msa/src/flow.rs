use anyhow::{anyhow, Result};

use crate::{
    mc::do_mc_auth,
    microsoft::{get_auth_token, msa_code},
    profile::get_profile,
    state::MsaState,
    verify::has_ownership,
    xbox::do_xbox_auth,
    xsts::do_xsts_auth,
};

pub async fn do_auth() -> Result<MsaState> {
    info!("Getting MSA code...");

    let code = msa_code()?;

    info!("Getting auth token...");

    let token = get_auth_token(&code).await?;

    info!("Getting Xbox token...");

    let xbox = do_xbox_auth(token).await?;

    info!("Getting XSTS token...");

    let xsts = do_xsts_auth(&xbox.0).await?;

    info!("Checking integrity...");

    let integrity = xbox.1 == xsts.1;

    if !integrity {
        return Err(anyhow!("Token integrity not valid!"));
    }

    info!("Getting Minecraft token...");

    let token = do_mc_auth(xsts).await?;

    info!("Checking ownership...");

    if !has_ownership(&token).await? {
        return Err(anyhow!("User does not own Minecraft!"));
    }

    info!("Getting profile...");

    let profile = get_profile(&token).await?;

    info!("Authenticated as: {}", profile.name);

    Ok(MsaState {
        access_token: token,
        player_name: profile.name,
        uuid: profile.id,
        xuid: String::new(),
    })
}
