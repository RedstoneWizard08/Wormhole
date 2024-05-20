// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use std::path::PathBuf;

use anyhow::Result;

use crate::{
    fabric::{
        get_fabric_launchwrapper, get_fabric_launchwrapper_artifact, get_fabric_loader,
        get_fabric_versions,
    },
    maven::artifact::download_maven_artifacts,
    test_util::library_download_callback,
};

#[tokio::test]
#[serial_test::serial]
pub async fn test_download_fabric_libs() -> Result<()> {
    let root = PathBuf::from("./test/libraries");
    let latest = get_fabric_versions().await?.versioning.latest;
    let config = get_fabric_launchwrapper(&latest).await?;

    let mut libs = vec![
        get_fabric_loader(&latest),
        get_fabric_launchwrapper_artifact(&latest),
    ];

    libs.extend(config.libraries.all());

    download_maven_artifacts(&root, libs, Some(Box::new(library_download_callback))).await?;

    Ok(())
}
