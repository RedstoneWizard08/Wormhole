#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchWrapperArgs {
    pub client: Vec<String>,
    pub common: Vec<String>,
    pub server: Vec<String>,
}
