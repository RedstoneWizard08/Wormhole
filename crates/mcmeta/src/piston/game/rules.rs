use std::{
    collections::HashMap,
    env::consts::{ARCH, OS},
};

use super::platform::{Arch, OperatingSystem};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsRule {
    pub name: Option<OperatingSystem>,
    pub arch: Option<Arch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub action: String,
    pub features: Option<HashMap<String, bool>>,
    pub os: Option<OsRule>,
}

impl Rule {
    pub fn should_apply(&self, features: &Vec<String>) -> bool {
        let mut ok = true;

        if let Some(feats) = &self.features {
            for (feat, enabled) in feats {
                if !(*enabled && features.contains(feat)) {
                    ok = false;
                }

                if !*enabled && features.contains(feat) {
                    ok = false;
                }
            }
        }

        if let Some(os) = &self.os {
            if let Some(name) = os.name {
                ok = match name {
                    OperatingSystem::Linux => OS == "linux",
                    OperatingSystem::Mac => OS == "macos",
                    OperatingSystem::Windows => OS == "windows",
                };
            }

            if let Some(arch) = os.arch {
                ok = match arch {
                    Arch::Amd64 => ARCH == "x86_64" || ARCH == "x86",
                    Arch::Arm64 => ARCH == "aarch64" || ARCH == "arm",
                };
            }
        }

        ok
    }
}
