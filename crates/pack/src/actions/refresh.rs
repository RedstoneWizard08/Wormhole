use std::{fs, path::PathBuf};

use anyhow::Result;
use ignore::WalkBuilder;
use sha1::{Digest, Sha1};

use crate::models::{
    index::{IndexFile, PackIndex},
    pack::Pack,
};

// Way to go, ignore!
// On 9/17/2024 this algorithm was able to index the ENTIRE Wormhole
// repo in under 2 seconds! (this does not include things in .gitignore)

impl Pack {
    pub fn refresh(&mut self, dir: Option<PathBuf>) -> Result<()> {
        let dir = dir.unwrap_or(PathBuf::from("."));
        let index_path = dir.join(&self.index.path);
        let mut index = PackIndex::read(&index_path)?;

        index.files = Vec::new();

        let files = WalkBuilder::new(&dir)
            .add_custom_ignore_filename(".whignore")
            .build();

        for file in files {
            if let Ok(file) = file {
                if !file.metadata()?.is_file() {
                    continue;
                }

                let path = file.path();
                let data = fs::read(path)?;
                let mut hasher = Sha1::new();

                hasher.update(data);

                let sha1 = hex::encode(hasher.finalize());

                index.files.push(IndexFile {
                    path: path.strip_prefix(&dir)?.to_str().unwrap().into(),
                    sha1,
                });
            }
        }

        let enc = index.encode()?;
        let mut hasher = Sha1::new();

        hasher.update(&enc);
        fs::write(index_path, enc)?;

        self.index.sha1 = hex::encode(hasher.finalize());
        self.save(Some(dir))?;

        Ok(())
    }
}
