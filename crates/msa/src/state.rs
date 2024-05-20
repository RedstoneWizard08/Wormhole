use anyhow::Result;

use crate::flow::do_auth;

static mut MSA_STATE: MsaState = MsaState::const_default();

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
        println!("Starting MSA flow...");

        if Self::get() == Self::const_default() {
            let res = do_auth().await.unwrap();
            
            Self::set(res);
        }

        Ok(Self::get())
    }
}
