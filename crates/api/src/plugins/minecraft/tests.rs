// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use data::instance::Instance;
#[cfg(test)]
use mcmeta::cmd::modded::ModLoader;
use msa::state::MsaState;
use whcore::manager::CoreManager;

use crate::{plugins::minecraft::manager::MinecraftManager, test_util::library_download_callback};

#[tokio::test]
pub async fn test_it_works() -> anyhow::Result<()> {
    let inst = Instance {
        id: None,
        cache_dir: CoreManager::get()
            .game_cache_dir("minecraft")
            .to_str()
            .unwrap()
            .into(),
        install_dir: CoreManager::get()
            .game_data_dir("minecraft")
            .to_str()
            .unwrap()
            .into(),
        data_dir: CoreManager::get()
            .game_data_dir("minecraft")
            .join("instances")
            .join("test")
            .to_str()
            .unwrap()
            .into(),
        description: String::new(),
        name: "Test Instance".into(),
        created: 0,
        updated: 0,
        game_id: 432,
    };

    let mut mgr =
        MinecraftManager::load_or_create(inst.data_dir(), &ModLoader::vanilla_latest().await?)
            .await?;

    mgr.install_loader(
        &ModLoader::fabric_latest().await?,
        &Some(Box::new(library_download_callback)),
    )
    .await?;

    mgr.launch(&MsaState::init().await?, &inst)
        .await?
        .wait()
        .await?;

    Ok(())
}
