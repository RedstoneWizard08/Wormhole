use serde::{Deserialize, Serialize};

pub mod export;
pub mod func;
pub mod router;

#[cfg(feature = "axum")]
pub mod axum;

#[cfg(feature = "tauri")]
pub mod tauri;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Method {
    Create,
    Read,
    Update,
    Delete,
    Error,
}

impl Method {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Create => "Create",
            Self::Read => "Read",
            Self::Update => "Update",
            Self::Delete => "Delete",
            Self::Error => "Error",
        }
    }
}
