#![warn(missing_docs, rustdoc::broken_intra_doc_links)]
//! # Plugins for Wormhole.
//!
//! This implements all of Wormhole's plugins and game supports.

#[macro_use]
pub extern crate async_trait;

#[macro_use]
extern crate tracing;

pub extern crate api;
pub extern crate base64;
pub extern crate const_format;
pub extern crate query;
pub extern crate whcore;
