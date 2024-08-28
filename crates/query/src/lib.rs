#![feature(associated_type_defaults)]
#![allow(unused_imports)]

use std::sync::Arc;

use data::prisma::PrismaClient;
use once_cell::sync::Lazy;

#[macro_use]
pub extern crate anyhow;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate specta;

#[macro_use]
extern crate envcrypt;

#[macro_use]
extern crate async_trait;

pub(crate) const CURSEFORGE_KEY: Lazy<&str> = Lazy::new(|| envc!("CURSEFORGE_KEY"));
pub(crate) const MODRINTH_KEY: Lazy<Option<&str>> = Lazy::new(|| option_envc!("MODRINTH_KEY"));

#[allow(unused)]
pub(crate) const NEXUS_API_KEY: Lazy<&str> = Lazy::new(|| envc!("NEXUS_API_KEY"));
