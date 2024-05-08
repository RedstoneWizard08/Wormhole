use std::{collections::HashMap, env::consts::OS};

use crate::maven::coord::MavenCoordinate;

use super::{download::LibDownloadRef, rules::Rule};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LibraryDownloads {
    pub artifact: Option<LibDownloadRef>,
    pub classifiers: Option<HashMap<String, LibDownloadRef>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryRef {
    pub downloads: Option<LibraryDownloads>,
    pub name: String,
    pub rules: Option<Vec<Rule>>,
    pub url: Option<String>,
}

impl LibraryRef {
    pub fn should_download(&self, features: &Vec<String>) -> bool {
        if let Some(rules) = &self.rules {
            rules.iter().all(|rule| rule.should_apply(features))
        } else {
            true
        }
    }

    pub fn is_native(&self) -> bool {
        MavenCoordinate::from(&self.name)
            .classifier
            .unwrap_or_default()
            .contains("natives")
    }
}

pub fn should_download_classifier(key: impl AsRef<str>) -> bool {
    match key.as_ref() {
        "natives-linux" => OS == "linux",
        "natives-osx" => OS == "macos",
        "natives-windows" => OS == "windows",

        _ => unreachable!(),
    }
}
