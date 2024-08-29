use std::{collections::HashMap, fs, path::PathBuf};
use anyhow::Result;
use tokio_stream::StreamExt;
use crate::vanilla::manifest::libraries::Library;

pub struct LibraryInstaller {
    /// A list of listeners.
    /// Fn(output_path, downloaded, total_bytes)
    listeners: Vec<Box<dyn Fn(&PathBuf, u64, u64)>>,
}

impl LibraryInstaller {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }

    pub fn add_listener<T>(&mut self, func: T) -> &mut Self where T: Fn(&PathBuf, u64, u64) + 'static {
        self.listeners.push(Box::new(func));
        self
    }

    /// Download library files into a root directory.
    /// 
    /// Args:
    /// - root: The root of the library folder.
    /// - libs: The libraries to download.
    /// - features: Enabled launcher features.
    pub async fn install_libraries(&self, root: PathBuf, libs: Vec<Library>, features: &HashMap<String, bool>) -> Result<()> {
        for lib in libs {
            let files = lib.get_files(features);

            for file in files {
                let path = root.join(file.path());

                if path.exists() {
                    self.notify_progress(&path, 1, 1);
                    continue;
                }

                let mut tmp_path = path.clone();
                let parent = path.parent().unwrap();

                tmp_path.add_extension(".tmp");
                fs::create_dir_all(parent)?;

                let req = reqwest::get(file.url).await?;
                let total = req.content_length().unwrap_or_default();
                let mut stream = req.bytes_stream();
                let mut data = Vec::new();

                while let Some(Ok(chunk)) = stream.next().await {
                    data.extend(chunk);
                    self.notify_progress(&path, data.len() as u64, total);
                }

                fs::write(&tmp_path, data)?;
                fs::rename(tmp_path, path)?;
            }
        }

        Ok(())
    }

    fn notify_progress(&self, path: &PathBuf, downloaded: u64, total: u64) {
        for listener in &self.listeners {
            listener(path, downloaded, total);
        }
    }
}
