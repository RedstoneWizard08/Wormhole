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
