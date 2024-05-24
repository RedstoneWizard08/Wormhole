use std::{collections::HashSet, path::PathBuf};

use anyhow::Result;
use whcore::manager::CoreManager;

use crate::{
    cmd_string_fix,
    forge::processor::resolve_jar,
    maven::coord::MavenCoordinate,
    piston::{game::manifest::GameManifest, get_features},
    quilt::intermediary::get_intermediary,
};

use super::{modded::ModLoader, options::LaunchOptions};

pub async fn fix_argument(
    arg: impl AsRef<str>,
    game_dir: &PathBuf,
    profile: &GameManifest,
    loader: &ModLoader,
    opts: &LaunchOptions,
) -> Result<String> {
    let mut arg = arg.as_ref().to_string();

    let mc_dir = CoreManager::get().game_data_dir("minecraft");
    let libs_dir = mc_dir.join("libraries");
    let assets_dir = mc_dir.join("assets").join(loader.mc_version());
    let natives_dir = mc_dir.join("natives");
    let mut classpath = Vec::new();

    match loader.clone() {
        ModLoader::Quilt(_, _) | ModLoader::Fabric(_, _) | ModLoader::Vanilla(_) => {
            classpath.push(
                libs_dir
                    .join(
                        MavenCoordinate::from(format!(
                            "net.minecraft:client:{}",
                            loader.mc_version()
                        ))
                        .path(),
                    )
                    .to_str()
                    .unwrap()
                    .to_string(),
            );
        }

        _ => {}
    };

    if let ModLoader::Quilt(_, _) = loader {
        let it = get_intermediary(loader.mc_version()).await?;

        classpath.push(
            libs_dir
                .join(it.coordinate().path())
                .to_str()
                .unwrap()
                .to_string(),
        );
    }

    for lib in &profile.libraries {
        if !lib.should_download(&get_features()) {
            continue;
        }

        classpath.push(resolve_jar(&libs_dir, lib.name.clone()));
    }

    let classpath = classpath
        .iter()
        .cloned()
        .collect::<HashSet<_>>()
        .iter()
        .cloned()
        .collect::<Vec<_>>();

    // Libraries
    cmd_string_fix!(arg, opts, natives_directory -> p:natives_dir);
    cmd_string_fix!(arg, opts, libraries_directory -> p:libs_dir);
    cmd_string_fix!(arg, opts, library_directory -> p:libs_dir);
    cmd_string_fix!(arg, opts, game_directory -> p:game_dir);

    // Assets
    cmd_string_fix!(arg, opts, assets_root -> p:assets_dir);
    cmd_string_fix!(arg, opts, assets_index_name -> profile.asset_index.id);

    // Launcher
    cmd_string_fix!(arg, opts, launcher_name -> o:launcher_name);
    cmd_string_fix!(arg, opts, launcher_version -> o:launcher_version);

    // Java stuff
    cmd_string_fix!(arg, opts, memory -> o:memory);

    #[cfg(not(windows))]
    {
        cmd_string_fix!(arg, opts, classpath -> classpath.join(":"));
        cmd_string_fix!(arg, opts, classpath_separator -> s:":");
    }

    #[cfg(windows)]
    {
        cmd_string_fix!(arg, opts, classpath -> classpath.join(";"));
        cmd_string_fix!(arg, opts, classpath_separator -> s:";");
    }

    // Player stuff
    cmd_string_fix!(arg, opts, auth_player_name -> o:player_name);
    cmd_string_fix!(arg, opts, auth_session -> o:access_token);
    cmd_string_fix!(arg, opts, auth_uuid -> o:uuid);
    cmd_string_fix!(arg, opts, auth_access_token -> o:access_token);
    cmd_string_fix!(arg, opts, clientid -> o:access_token);
    cmd_string_fix!(arg, opts, auth_xuid -> o:xuid);
    cmd_string_fix!(arg, opts, user_type -> s:"msa");

    // Version
    cmd_string_fix!(arg, opts, version_name -> profile.id);
    cmd_string_fix!(arg, opts, version_type -> s:"release");

    if arg.starts_with("-Dlog4j.configurationFile=") {
        if let Some(config) = &profile.logging {
            if let Some(client) = &config.client {
                arg = format!(
                    "-Dlog4j.configurationFile={}",
                    libs_dir.join(&client.file.id).to_str().unwrap()
                );
            }
        }
    }

    Ok(arg)
}

pub async fn fix_args(
    args: Vec<impl AsRef<str>>,
    root: &PathBuf,
    profile: &GameManifest,
    loader: &ModLoader,
    opts: &LaunchOptions,
) -> Result<Vec<String>> {
    let mut out = Vec::new();

    for arg in args {
        out.push(fix_argument(arg, root, profile, loader, opts).await?.replace("\\", "/"));
    }

    Ok(out)
}
