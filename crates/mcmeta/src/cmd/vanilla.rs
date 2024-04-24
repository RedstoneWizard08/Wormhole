use std::path::PathBuf;

use crate::cmd_string_fix;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchOptions {
    /// The player's username.
    /// Ex: "Notch"
    pub player_name: String,

    /// The player's session ID.
    /// This can be None since newer versions don't support this.
    /// This is only needed for old versions.
    pub session_id: Option<String>,

    /// The version name.
    /// Ex: "1.20.4"
    pub version: String,

    /// The game directory.
    /// Ex: "~/.minecraft"
    pub game_dir: PathBuf,

    /// Where the assets are stored.
    /// Ex: "~/.local/share/Wormhole/data/minecraft/assets"
    pub assets_dir: PathBuf,

    /// The asset index's name.
    /// Ex: (for 1.20.4) "12"
    pub assets_index: String,

    /// The player's UUID.
    /// Ex: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
    pub uuid: String,

    /// The player's access token.
    /// This is received through MSA (auth).
    pub access_token: String,

    /// The client's ID.
    pub client_id: String,

    /// The player's XUID (Xbox User ID).
    pub xuid: String,

    /// The player's user type.
    pub user_type: String,

    /// The version type.
    /// Ex: "Release"
    pub version_type: String,

    /// The natives directory.
    /// Ex: "~/.local/share/Wormhole/data/minecraft/natives"
    pub natives_dir: PathBuf,

    /// The libraries directory.
    /// Ex: "~/.local/share/Wormhole/data/minecraft/libraries"
    pub libs_dir: PathBuf,

    /// The launcher name.
    /// Ex: "Wormhole"
    pub launcher_name: String,

    /// The launcher version.
    /// Ex: "2.0.0"
    pub launcher_version: String,

    /// The classpath.
    /// Ex: vec!["client.jar", lib.jar", "lwjgl.jar", ...]
    pub classpath: Vec<String>,

    /// The memory to launch with.
    /// Ex: 4096 (MB) for 4 GB
    pub memory: u32,
}

pub fn fix_argument(arg: impl AsRef<str>, opts: &LaunchOptions) -> String {
    let mut arg = arg.as_ref().to_string();

    cmd_string_fix!(arg, opts, auth_player_name -> player_name);
    cmd_string_fix!(arg, opts, auth_session -> o:session_id);
    cmd_string_fix!(arg, opts, version_name -> version);
    cmd_string_fix!(arg, opts, game_directory -> p:game_dir);
    cmd_string_fix!(arg, opts, assets_root -> p:assets_dir);
    cmd_string_fix!(arg, opts, assets_index_name -> assets_index);
    cmd_string_fix!(arg, opts, auth_uuid -> uuid);
    cmd_string_fix!(arg, opts, auth_access_token -> access_token);
    cmd_string_fix!(arg, opts, clientid -> client_id);
    cmd_string_fix!(arg, opts, auth_xuid -> xuid);
    cmd_string_fix!(arg, opts, user_type -> user_type);
    cmd_string_fix!(arg, opts, version_type -> version_type);
    cmd_string_fix!(arg, opts, natives_directory -> p:natives_dir);
    cmd_string_fix!(arg, opts, launcher_name -> launcher_name);
    cmd_string_fix!(arg, opts, launcher_version -> launcher_version);
    cmd_string_fix!(arg, opts, memory -> memory);
    cmd_string_fix!(arg, opts, libraries_directory -> p:libs_dir);
    cmd_string_fix!(arg, opts, classpath_separator -> s:":");

    arg.replace("${classpath}", &opts.classpath.join(":"))
}

pub fn fix_args(args: Vec<impl AsRef<str>>, opts: LaunchOptions) -> Vec<String> {
    args.iter()
        .map(|v| fix_argument(v, &opts))
        .collect::<Vec<_>>()
}
