#![feature(async_closure)]
#![warn(missing_docs, rustdoc::broken_intra_doc_links)]

//! # Wormhole Web UI
//!
//! This is also a library crate so it's API can be used elsewhere.
//!
//! ## About
//!
//! This exists because I do a lot of development on my Chromebook,
//! and I can't run GUI-based apps there. So, I needed a way to run
//! the app with full capabilities and without having to install anything
//! on my Chromebook.
//!
//! This works by using a HTTP server to serve a route (`/_tauri/invoke`)
//! that allows the frontend to invoke commands on the backend. The backend
//! is responsible for the actual work of the app. This also uses my [`glue`]
//! library to proxy the frontend's development server, allowing it to be
//! accessed through a single port.
//!
//! As a bonus, this can be used to remotely manage mods and instances, and
//! even build modpacks when you don't have access to a full computer. I'll
//! probably work on mobile support for this UI in the future, so you can even
//! manage stuff from your phone. The UI is the exact same as the full GUI app.
//!
//! This also comes with a CLI that allows you to quickly and easily run the
//! server. In a release build, it'll also bundle a build of the UI inside the
//! executable, so you don't need any other files present to run it.

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate serde;

pub mod cli;
pub mod code;
pub mod glue;
pub mod log;
pub mod route;
pub mod router;
pub mod server;
pub mod state;
pub mod ws;
