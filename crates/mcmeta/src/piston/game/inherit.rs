use anyhow::{anyhow, Result};

use crate::piston::manifest::get_manifest;

use super::{
    args::GameArgs, get_game_manifest, library::LibraryRef, logging::LoggingConfigs,
    manifest::GameManifest,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InheritedGameManifest {
    pub arguments: GameArgs,
    pub id: String,
    pub inherits_from: String,
    pub libraries: Vec<LibraryRef>,
    pub logging: Option<LoggingConfigs>,
    pub main_class: String,
    pub release_time: String,
    pub time: String,

    #[serde(rename = "type")]
    pub kind: String,
}

impl InheritedGameManifest {
    pub async fn resolve(&self) -> Result<GameManifest> {
        let version = get_manifest()
            .await?
            .find(&self.inherits_from)
            .ok_or(anyhow!("Cannot find the inherited version!"))?;

        let mut manifest = get_game_manifest(version.url).await?;
        let mut my_arguments = self.arguments.clone();

        if let Some(args) = manifest.arguments.as_mut() {
            args.game.append(&mut my_arguments.game);
            args.jvm.append(&mut my_arguments.jvm);
        }

        manifest.id = self.id.clone();
        manifest.libraries.append(&mut self.libraries.clone());

        if let Some(config) = &self.logging {
            manifest.logging = Some(config.clone());
        }

        manifest.main_class = self.main_class.clone();
        manifest.release_time = self.release_time.clone();
        manifest.time = self.time.clone();
        manifest.kind = self.kind.clone();

        Ok(manifest)
    }
}
