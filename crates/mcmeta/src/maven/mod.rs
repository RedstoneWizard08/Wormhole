use anyhow::Result;
use artifact::Artifact;
use metadata::Metadata;

pub mod artifact;
pub mod metadata;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct MavenRepo {
    root: &'static str,
}

impl MavenRepo {
    pub const fn new(root: &'static str) -> MavenRepo {
        MavenRepo { root }
    }

    pub async fn get_versions(&self, artifact: impl Into<Artifact>) -> Result<Vec<Artifact>> {
        let artifact = artifact.into();
        let url = format!(
            "{}/{}/maven-metadata.xml",
            self.root,
            artifact.clone().base_url_versionless()
        );
        let xml = reqwest::get(url).await?.text().await?;
        let meta = quick_xml::de::from_str::<Metadata>(&xml)?;

        Ok(meta
            .versioning
            .versions
            .version
            .iter()
            .map(|it| artifact.clone().set_version(it))
            .collect())
    }

    pub async fn get_latest(&self, artifact: impl Into<Artifact>) -> Result<Artifact> {
        let artifact = artifact.into();
        let url = format!(
            "{}/{}/maven-metadata.xml",
            self.root,
            artifact.clone().base_url_versionless()
        );
        let xml = reqwest::get(url).await?.text().await?;
        let meta = quick_xml::de::from_str::<Metadata>(&xml)?;

        Ok(artifact.set_version(meta.versioning.latest))
    }

    pub async fn get_latest_release(&self, artifact: impl Into<Artifact>) -> Result<Artifact> {
        let artifact = artifact.into();
        let url = format!(
            "{}/{}/maven-metadata.xml",
            self.root,
            artifact.clone().base_url_versionless()
        );
        let xml = reqwest::get(url).await?.text().await?;
        let meta = quick_xml::de::from_str::<Metadata>(&xml)?;

        Ok(artifact.set_version(meta.versioning.release))
    }

    pub fn get_artifact_url(&self, artifact: impl Into<Artifact>) -> String {
        format!("{}/{}", self.root, artifact.into().url())
    }
}
