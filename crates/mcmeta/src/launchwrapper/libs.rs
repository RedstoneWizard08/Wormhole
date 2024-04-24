use crate::maven::artifact::MavenArtifact;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchWrapperLibs {
    pub client: Vec<MavenArtifact>,
    pub common: Vec<MavenArtifact>,
    pub server: Vec<MavenArtifact>,
    pub development: Vec<MavenArtifact>,
}

impl LaunchWrapperLibs {
    pub fn all(&self) -> Vec<MavenArtifact> {
        let mut all = Vec::new();

        all.extend(self.client.clone());
        all.extend(self.common.clone());
        all.extend(self.server.clone());
        all.extend(self.development.clone());

        all
    }
}
