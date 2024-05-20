// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

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
