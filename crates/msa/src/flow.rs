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
    let code = msa_code()?;
    dbg!(&code);
    let token = get_auth_token(&code).await?;
    dbg!(&token);
    let xbox = do_xbox_auth(token).await?;
    dbg!(&xbox);
    let xsts = do_xsts_auth(&xbox.0).await?;
    dbg!(&xsts);
    let integrity = xbox.1 == xsts.1;
    dbg!(&integrity);

    if !integrity {
        return Err(anyhow!("Token integrity not valid!"));
    }

    let token = do_mc_auth(xsts).await?;
    dbg!(&token);

    if !has_ownership(&token).await? {
        return Err(anyhow!("User does not own Minecraft!"));
    }

    let profile = get_profile(&token).await?;
    dbg!(&profile);

    println!("Authenticated as: {}", profile.name);

    Ok(MsaState {
        access_token: token,
        player_name: profile.name,
        uuid: profile.id,
        xuid: String::new(),
    })
}
