use anyhow::{anyhow, Result};

use crate::{
    mc::do_mc_auth,
    microsoft::{get_auth_token, msa_code},
    xbox::do_xbox_auth,
    xsts::do_xsts_auth,
};

pub async fn do_auth() -> Result<String> {
    let code = msa_code()?;
    let token = get_auth_token(&code).await?;
    let xbox = do_xbox_auth(token).await?;
    let xsts = do_xsts_auth(xbox.0).await?;
    let integrity = xbox.1 == xsts.1;

    if !integrity {
        return Err(anyhow!("Token integrity not valid!"));
    }

    let token = do_mc_auth(xsts).await?;

    Ok(token)
}
