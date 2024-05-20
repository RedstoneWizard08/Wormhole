// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

pub mod install;

use std::env::consts::{ARCH, OS};

use anyhow::Result;
use reqwest::Client;
use serde_json::Value;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumString, Display)]
pub enum OperatingSystem {
    #[strum(serialize = "windows")]
    Windows,

    #[strum(serialize = "linux")]
    Linux,

    #[strum(serialize = "mac")]
    Mac,
}

impl OperatingSystem {
    pub fn detect() -> Self {
        match OS {
            "linux" => Self::Linux,
            "macos" => Self::Mac,
            "windows" => Self::Windows,

            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumString, Display)]
pub enum Arch {
    #[strum(serialize = "x64")]
    Amd64,

    #[strum(serialize = "x86")]
    I686,

    #[strum(serialize = "aarch64")]
    Arm64,

    #[strum(serialize = "arm")]
    Arm,
}

impl Arch {
    pub fn detect() -> Self {
        match ARCH {
            "x86_64" => Self::Amd64,
            "x86" => Self::I686,
            "arm" => Self::Arm,
            "aarch64" => Self::Arm64,

            _ => unimplemented!(),
        }
    }
}

pub async fn get_release_name(java: i32, os: OperatingSystem, arch: Arch) -> Result<String> {
    let os = format!("{}", os);
    let arch = format!("{}", arch);
    let java = format!("[{},{})", java, java + 1);
    let client = Client::new();

    let res = client
        .get("https://api.adoptium.net/v3/info/release_names")
        .query(&[
            ("architecture", arch.as_str()),
            ("heap_size", "normal"),
            ("image_type", "jre"),
            ("jvm_impl", "hotspot"),
            ("os", os.as_str()),
            ("page", "0"),
            ("page_size", "1"),
            ("project", "jdk"),
            ("release_type", "ga"),
            ("semver", "false"),
            ("sort_method", "DATE"),
            ("sort_order", "DESC"),
            ("vendor", "eclipse"),
            ("version", java.as_str()),
        ])
        .send()
        .await?
        .json::<Value>()
        .await?;

    // I'm too lazy to make a struct, so here we are.
    let res = res
        .get("releases")
        .unwrap()
        .as_array()
        .unwrap()
        .first()
        .unwrap()
        .as_str()
        .unwrap();

    Ok(res.to_string())
}

pub async fn get_release_url(java: i32, os: OperatingSystem, arch: Arch) -> Result<String> {
    Ok(format!("https://api.adoptium.net/v3/binary/version/{}/{}/{}/jre/hotspot/normal/eclipse?project=jdk", get_release_name(java, os, arch).await?, os, arch))
}
