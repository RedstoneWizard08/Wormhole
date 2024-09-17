#![feature(buf_read_has_data_left)]

//! # WHPack
//!
//! Wormhole's modpack management & creation module.

#[macro_use]
extern crate serde;

#[macro_use]
extern crate specta;

pub extern crate anyhow;

pub mod actions;
pub mod macros;
pub mod models;
