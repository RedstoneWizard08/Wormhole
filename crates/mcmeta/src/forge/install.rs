use std::collections::HashMap;

use crate::piston::game::library::LibraryRef;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallProfile {
    pub spec: u8,
    pub profile: String,
    pub version: String,
    pub path: Option<String>,
    pub minecraft: String,
    pub server_jar_path: String,
    pub data: HashMap<String, Sided<String>>,
    pub processors: Vec<Processor>,
    pub libraries: Vec<LibraryRef>,
    pub icon: String,
    pub json: String,
    pub logo: String,
    pub mirror_list: String,
    pub welcome: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Processor {
    pub sides: Option<Vec<String>>,
    pub jar: String,
    pub classpath: Vec<String>,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sided<T> {
    pub client: T,
    pub server: T,
}
