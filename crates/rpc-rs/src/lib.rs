//! # rpc-rs
//!
//! A simple IPC/RPC framework for Rust and TypeScript,
//! built for Tauri and web apps.

#![warn(missing_docs)]
#![feature(associated_type_defaults, trait_alias, async_closure)]

pub mod events;
pub mod filtered;
pub mod macros;
pub mod module;
pub mod proc;
pub mod router;
pub mod util;

pub use module::{Module, ModuleBuilder};
pub use router::{Method, Router};
pub use rpc_rs_macros::Event;
