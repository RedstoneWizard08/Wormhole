// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

#![feature(associated_type_defaults)]
#![allow(unused_imports)]

use once_cell::sync::Lazy;

#[macro_use]
extern crate async_trait;

#[macro_use]
pub extern crate anyhow;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate specta;

#[macro_use]
extern crate envcrypt;

pub(crate) const CURSEFORGE_KEY: Lazy<&str> = Lazy::new(|| envc!("CURSEFORGE_KEY"));
pub(crate) const MODRINTH_KEY: Lazy<Option<&str>> = Lazy::new(|| option_envc!("MODRINTH_KEY"));

#[allow(unused)]
pub(crate) const NEXUS_API_KEY: Lazy<&str> = Lazy::new(|| envc!("NEXUS_API_KEY"));

/// This backend is currently not implemented.
pub mod nexus;

pub mod ckan;
pub mod conv;
pub mod curse;
pub mod macros;
pub mod mod_;
pub mod modrinth;
pub mod source;
pub mod spacedock;
pub mod thunderstore;

#[async_trait]
pub trait IntoAsync<T> {
    async fn into_async(self) -> T;
}
