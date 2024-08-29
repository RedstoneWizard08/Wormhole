use std::{
    collections::HashMap,
    env::consts::{ARCH, OS},
};

use crate::maven::artifact::Artifact;

use super::rules::Rule;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct LibraryFile {
    pub path: String,
    pub sha1: Option<String>,
    pub size: Option<u64>,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct LibraryDownloads {
    pub artifact: Option<LibraryFile>,
    // could be: windows-natives, linux-natives, macos-natives, linux-x86_64
    #[serde(default)]
    pub classifiers: HashMap<String, LibraryFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Library {
    pub downloads: LibraryDownloads,
    pub name: String,
    #[serde(default)]
    pub rules: Vec<Rule>,
}

impl LibraryFile {
    pub fn path(&self) -> String {
        if self.path.ends_with(".jar") || self.path.contains("/") {
            self.path.clone()
        } else {
            Artifact::new(&self.path).url()
        }
    }
}

impl Library {
    pub fn get_files(&self, features: &HashMap<String, bool>) -> Vec<LibraryFile> {
        if self.rules.iter().all(|v| v.applies(features)) {
            self.downloads.get_files()
        } else {
            Vec::new()
        }
    }
}

impl LibraryDownloads {
    pub fn get_files(&self) -> Vec<LibraryFile> {
        let mut files = Vec::new();

        if let Some(val) = self.classifiers.get("windows-natives") {
            if OS == "windows" {
                files.push(val.clone());
            }
        }

        if let Some(val) = self.classifiers.get("macos-natives") {
            if OS == "macos" {
                files.push(val.clone());
            }
        }

        if let Some(val) = self.classifiers.get("linux-natives") {
            if OS == "linux" {
                files.push(val.clone());
            }
        }

        if let Some(val) = self.classifiers.get("linux-x86_64") {
            if OS == "linux" && ARCH == "x86_64" {
                files.push(val.clone());
            }
        }

        if let Some(val) = &self.artifact {
            files.push(val.clone());
        }

        files
    }
}
