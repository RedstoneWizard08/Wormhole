#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TweakerConfig {
    pub tweakers: LaunchWrapperTweakers,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchWrapperTweakers {
    pub client: Vec<String>,
    pub common: Vec<String>,
    pub server: Vec<String>,
}
