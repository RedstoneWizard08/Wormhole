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
