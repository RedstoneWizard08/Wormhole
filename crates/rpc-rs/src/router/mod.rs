//! The core router module.

use serde::{Deserialize, Serialize};

pub mod export;
pub mod func;
pub mod router;

#[cfg(feature = "axum")]
pub mod axum;

#[cfg(feature = "tauri")]
pub mod tauri;

/// A request method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Method {
    /// Create an object.
    Create,

    /// Read an object.
    Read,

    /// Update an object.
    Update,

    /// Delete an object.
    Delete,

    /// Only used when an error occurs.
    Error,
}

impl Method {
    /// Get this [`Method`] as a [`str`].
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

pub use router::Router;
