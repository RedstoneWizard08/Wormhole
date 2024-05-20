// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

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
    let token = get_auth_token(&code).await?;
    let xbox = do_xbox_auth(token).await?;
    let xsts = do_xsts_auth(&xbox.0).await?;
    let integrity = xbox.1 == xsts.1;

    if !integrity {
        return Err(anyhow!("Token integrity not valid!"));
    }

    let token = do_mc_auth(xsts).await?;

    if !has_ownership(&token).await? {
        return Err(anyhow!("User does not own Minecraft!"));
    }

    let profile = get_profile(&token).await?;

    Ok(MsaState {
        access_token: token,
        player_name: profile.name,
        uuid: profile.id,
        xuid: String::new(),
    })
}
