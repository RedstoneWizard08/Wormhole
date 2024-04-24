use std::path::PathBuf;

use super::{
    game::{library::should_download_classifier, manifest::GameManifest},
    get_features,
};

pub fn build_classpath(lib_dir: impl Into<PathBuf>, game: GameManifest) -> Vec<PathBuf> {
    let lib_dir = lib_dir.into();
    let mut res = Vec::new();

    for lib in game.libraries {
        if lib.should_download(&get_features()) {
            if let Some(artifact) = lib.downloads.artifact {
                res.push(artifact.path);
            }

            if let Some(classifiers) = lib.downloads.classifiers {
                for (key, lib) in classifiers {
                    if should_download_classifier(key) {
                        res.push(lib.path);
                    }
                }
            }
        }
    }

    res.iter().map(|v| lib_dir.join(v)).collect::<Vec<_>>()
}
