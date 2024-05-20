// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use anyhow::Result;

use crate::flow::do_auth;

static mut MSA_STATE: MsaState = MsaState::const_default();

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MsaState {
    /// The player's username.
    /// Ex: "Notch"
    pub player_name: String,

    /// The player's UUID.
    /// Ex: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
    pub uuid: String,

    /// The player's access token.
    /// This is received through MSA (auth).
    pub access_token: String,

    /// The player's XUID (Xbox User ID).
    pub xuid: String,
}

impl MsaState {
    pub const fn const_default() -> Self {
        Self {
            player_name: String::new(),
            uuid: String::new(),
            access_token: String::new(),
            xuid: String::new(),
        }
    }

    pub fn get() -> Self {
        unsafe { MSA_STATE.clone() }
    }

    pub fn set(new: MsaState) -> MsaState {
        unsafe {
            MSA_STATE = new;
            MSA_STATE.clone()
        }
    }

    pub async fn init() -> Result<MsaState> {
        Ok(Self::set(do_auth().await?))
    }
}
