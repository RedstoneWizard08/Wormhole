//! # Models
//!
//! WHPack follows the pack-config-index system introduced by Packwiz.
//! The pack.toml file will contain general information about the pack,
//! the index.lock file will contain a cached list of every file in the pack,
//! and the .whignore file will contain a gitignore-style list of files to ignore.

pub mod index;
pub mod pack;
