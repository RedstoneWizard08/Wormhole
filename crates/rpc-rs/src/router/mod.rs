use serde::{Deserialize, Serialize};

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
