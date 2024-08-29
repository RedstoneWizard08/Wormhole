use std::collections::HashMap;

use anyhow::Result;

use crate::{
    maven::{artifact::Artifact, MavenRepo},
    vanilla::{
        manifest::{
            args::Arguments,
            libraries::{Library, LibraryDownloads, LibraryFile},
            VersionManifest,
        },
        meta::PistonMetaVersionType,
        Vanilla,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallerManifest {
    pub id: String,
    pub time: String,
    pub release_time: Option<String>,
    #[serde(rename = "type")]
    pub kind: PistonMetaVersionType,
    pub main_class: Option<String>,
    pub inherits_from: Option<String>,
    pub arguments: Arguments,
    pub libraries: Vec<InstallerLibraryFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, Default)]
pub struct InstallerLibraryFile {
    pub name: String,
    pub sha1: Option<String>,
    pub size: Option<u64>,
    pub url: Option<String>,
}

impl InstallerLibraryFile {
    pub fn into_lib(self, maven: MavenRepo) -> LibraryFile {
        let mvn_url = self
            .url
            .clone()
            .map(MavenRepo::new)
            .unwrap_or(maven)
            .get_artifact_url(self.name.clone());

        let url = if let Some(url) = self.url {
            if url.starts_with("http") {
                url
            } else {
                mvn_url
            }
        } else {
            mvn_url
        };

        LibraryFile {
            path: Artifact::new(self.name).url(),
            url,
            size: self.size,
            sha1: self.sha1,
        }
    }
}

impl InstallerManifest {
    pub fn merge(&self, mut full: VersionManifest, maven: MavenRepo) -> VersionManifest {
        if let Some(main) = self.main_class.clone() {
            full.main_class = main;
        }

        if let Some(args) = &mut full.arguments {
            args.extend(self.arguments.clone());
        }

        for lib in &self.libraries {
            full.libraries.push(Library {
                name: lib.name.clone(),
                rules: Vec::new(),

                downloads: LibraryDownloads {
                    artifact: Some(lib.clone().into_lib(maven.clone())),
                    classifiers: HashMap::new(),
                },
            });
        }

        full
    }

    pub async fn resolve(
        &self,
        inherits_from: String,
        maven: MavenRepo,
    ) -> Result<VersionManifest> {
        Ok(self.merge(Vanilla.get_manifest_for(inherits_from).await?, maven))
    }
}
