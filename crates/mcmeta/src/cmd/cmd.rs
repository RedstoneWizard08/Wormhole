use std::path::PathBuf;

use anyhow::Result;

use crate::piston::get_features;

use super::{args::fix_args, modded::ModLoader, options::LaunchOptions};

pub async fn build_launch_command(
    java: &PathBuf,
    root: &PathBuf,
    loader: &ModLoader,
    opts: LaunchOptions,
) -> Result<Vec<String>> {
    let mut args = Vec::new();
    let profile = loader.get_manifest().await?;

    args.push(java.to_str().unwrap().to_string());

    args.extend(
        fix_args(
            profile.java_args(&get_features()),
            &root,
            &profile,
            loader,
            &opts,
        )
        .await?,
    );

    args.push(profile.main_class.clone());

    args.extend(
        fix_args(
            profile.game_args(&get_features()),
            &root,
            &profile,
            loader,
            &opts,
        )
        .await?,
    );

    Ok(args)
}
