use std::collections::HashMap;
use serde_with::BoolFromInt;

pub type VersionApiResult = HashMap<String, HashMap<String, VersionFile>>;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Type)]
pub struct VersionFile {
    pub filename: String,
    pub filesize: String,
    pub md5: String,
    pub urls: VersionUrls,
    
    #[serde(default)]
    #[serde_as(as = "BoolFromInt")]
    pub latest: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Type)]
pub struct VersionUrls {
    pub cdn: String,
    pub local: String,
}
