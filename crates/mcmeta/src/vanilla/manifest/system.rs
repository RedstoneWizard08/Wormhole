#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Type, Default)]
#[serde(rename_all = "snake_case")]
pub enum OperatingSystem {
    Windows,
    Osx,
    Linux,
    #[default]
    Any,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Type, Default)]
#[serde(rename_all = "snake_case")]
pub enum Arch {
    X86,
    Arm,
    #[default]
    Any,
}
