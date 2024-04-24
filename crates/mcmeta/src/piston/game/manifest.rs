use super::{
    args::GameArgs, assets::AssetIndexRef, download::GameDownloads, java::JavaRef,
    library::LibraryRef, logging::LoggingConfigs,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameManifest {
    pub arguments: Option<GameArgs>,
    pub asset_index: AssetIndexRef,
    pub assets: String,
    pub compliance_level: i32,
    pub downloads: GameDownloads,
    pub id: String,
    pub java_version: JavaRef,
    pub libraries: Vec<LibraryRef>,
    pub logging: Option<LoggingConfigs>,
    pub main_class: String,
    pub minimum_launcher_version: i32,
    pub release_time: String,
    pub time: String,
    pub minecraft_args: Option<String>,

    #[serde(rename = "type")]
    pub kind: String,
}

impl GameManifest {
    pub fn game_args(&self, feats: &Vec<String>) -> Vec<String> {
        let mut out = self
            .arguments
            .as_ref()
            .map(|v| v.get_game_args(feats))
            .unwrap_or_default();

        if let Some(args) = &self.minecraft_args {
            out.append(&mut args.split(" ").map(|v| v.to_string()).collect::<Vec<_>>());
        }

        out
    }

    pub fn java_args(&self, feats: &Vec<String>) -> Vec<String> {
        let mut args = self
            .arguments
            .as_ref()
            .map(|v| v.get_jvm_args(feats))
            .unwrap_or_default();

        if let Some(config) = &self.logging {
            if let Some(client) = &config.client {
                args.push(client.argument.clone());
            }
        }

        args
    }
}
