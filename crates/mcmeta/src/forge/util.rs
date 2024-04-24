use crate::maven::artifact::MavenArtifact;

use super::FORGE_MAVEN;

pub fn get_artifact_ref(orig: String) -> MavenArtifact {
    let real = orig.trim_start_matches('[').trim_end_matches(']');

    MavenArtifact {
        name: real.into(),
        repo: FORGE_MAVEN.into(),
    }
}
