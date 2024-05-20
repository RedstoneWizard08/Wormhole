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
    cmd::{modded::ModLoader, options::LaunchOptions},
    test_util::library_download_callback,
};

#[tokio::test]
pub async fn test_cmd_gen_works() -> Result<()> {
    let it = ModLoader::neo_latest().await?;
    let game_dir = PathBuf::from("./test");

    it.install_to(game_dir.clone(), &Some(Box::new(library_download_callback)))
        .await?;

    let cmd = it
        .cmd(
            game_dir,
            LaunchOptions {
                ..Default::default()
            },
        )
        .await?;

    println!("{}", cmd.join(" "));

    Ok(())
}
