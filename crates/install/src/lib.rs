// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use anyhow::Result;
use api::plugin::Plugin;
use data::{conv::DbIntoArg, instance::Instance, Conn};
use extract::extract_file;
use magic::detect_file_type;
use query::mod_::{Mod, ModVersion};
use std::io::Write;
use tempfile::NamedTempFile;

pub mod extract;
pub mod magic;

#[macro_use]
extern crate anyhow;

pub async fn install_mod<T: Plugin>(
    db: &mut Conn,
    item: Mod,
    version: Option<ModVersion>,
    instance: Instance,
    plugin: T,
) -> Result<()> {
    let src = plugin.get_source(item.source).await.unwrap();
    let version = version.unwrap_or(src.get_latest_version(item.id.clone()).await?);

    let data = reqwest::get(
        src.get_download_url(item.id.clone(), Some(version.clone().id))
            .await?,
    )
    .await?;

    let data = data.bytes().await?.to_vec();
    let kind = detect_file_type(&data)?;
    let mut file = NamedTempFile::new()?;

    let _ = file.write(&data.into_boxed_slice())?;

    extract_file(
        db,
        file.path().into(),
        kind,
        instance.clone(),
        plugin.fallback(),
    )?;
    item.db_into_arg(db, (version, instance))?;

    Ok(())
}
