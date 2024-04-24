use crate::maven::artifact::MavenArtifact;

use super::NEOFORGE_MAVEN;

pub fn get_artifact_ref(orig: String) -> MavenArtifact {
    let real = orig.trim_start_matches('[').trim_end_matches(']');

    MavenArtifact {
        name: real.into(),
        repo: NEOFORGE_MAVEN.into(),
    }
}
