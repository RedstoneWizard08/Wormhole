use args::Arguments;
use assets::AssetIndex;
use download::Downloads;
use java::JavaVersion;
use libraries::Library;
use logging::LoggingConfigs;

use super::meta::PistonMetaVersionType;

pub mod args;
pub mod assets;
pub mod download;
pub mod java;
pub mod libraries;
pub mod logging;
pub mod rules;
pub mod system;

#[cfg(test)]
pub mod tests;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionManifest {
    pub arguments: Option<Arguments>, // Used in anything above 1.12.2
    pub minecraft_arguments: Option<String>, // This is used in 1.12.2 and below
    pub asset_index: AssetIndex,
    pub assets: String,
    pub compliance_level: Option<i32>, // Missing below 13w39a
    pub downloads: Downloads,
    pub id: String,
    pub java_version: Option<JavaVersion>, // Missing below 13w39a
    pub libraries: Vec<Library>,
    pub logging: Option<LoggingConfigs>, // Missing below 14w28a
    pub main_class: String,
    pub minimum_launcher_version: i32,
    pub release_time: String,
    pub time: String,
    #[serde(rename = "type")]
    pub kind: PistonMetaVersionType,
}
