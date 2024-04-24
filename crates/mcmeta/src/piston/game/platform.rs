#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum OperatingSystem {
    #[serde(rename = "windows")]
    Windows,

    #[serde(rename = "osx")]
    Mac,

    #[serde(rename = "linux")]
    Linux,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Arch {
    #[serde(rename = "x86")]
    Amd64,

    #[serde(rename = "arm64")]
    Arm64,
}
