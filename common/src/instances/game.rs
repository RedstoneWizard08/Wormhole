#[cfg(feature = "clap")]
use clap::ValueEnum;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Copy)]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
pub enum KSPGame {
    KSP1 = 3102,
    KSP2 = 22407,
}

impl KSPGame {
    pub fn from_id(id: i32) -> Option<Self> {
        match id {
            3102 => Some(KSPGame::KSP1),
            22407 => Some(KSPGame::KSP2),
            _ => None,
        }
    }

    pub fn as_i32(&self) -> i32 {
        return match self {
            KSPGame::KSP1 => 3102,
            KSPGame::KSP2 => 22407,
        };
    }
}
