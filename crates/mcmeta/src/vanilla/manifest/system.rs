use std::env::consts::{ARCH, OS};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Type, Default,
)]
#[serde(rename_all = "snake_case")]
pub enum OperatingSystem {
    Windows,
    Osx,
    Linux,
    #[default]
    Any,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Type, Default,
)]
#[serde(rename_all = "snake_case")]
pub enum Arch {
    X86,
    Arm,
    #[default]
    Any,
}

impl OperatingSystem {
    pub fn is_current(&self) -> bool {
        match OS {
            "linux" => *self == Self::Linux,
            "windows" => *self == Self::Windows,
            "macos" => *self == Self::Osx,
            _ => *self == Self::Any,
        }
    }
}

impl Arch {
    pub fn is_current(&self) -> bool {
        match ARCH {
            "x86" | "x86_64" => *self == Self::X86,
            "arm" | "aarch64" => *self == Self::Arm,
            _ => *self == Self::Any,
        }
    }
}
