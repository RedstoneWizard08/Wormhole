use std::path::PathBuf;
use specta::Type;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default, Type)]
pub struct Dirs {
    pub root: PathBuf,
    pub data: PathBuf,
    pub cache: PathBuf,
    pub temp: PathBuf,
}
