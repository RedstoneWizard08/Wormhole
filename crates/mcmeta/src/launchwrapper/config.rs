use serde_either::StringOrStruct;

use super::{
    args::LaunchWrapperArgs, libs::LaunchWrapperLibs, main_class::MainClassConfig,
    tweakers::TweakerConfig,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchWrapperConfig {
    pub version: u32,
    pub min_java_version: u32,
    pub libraries: LaunchWrapperLibs,
    pub arguments: Option<LaunchWrapperArgs>,
    pub launchwrapper: Option<TweakerConfig>,

    // Why is this camelCase and everything else isn't? Excuse me, consistency?
    #[serde(rename = "mainClass")]
    pub main_class: StringOrStruct<MainClassConfig>,
}
