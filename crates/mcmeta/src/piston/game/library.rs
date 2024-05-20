// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

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
