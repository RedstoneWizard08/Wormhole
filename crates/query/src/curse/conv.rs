use data::source::Sources;
use furse::structures::{file_structs::File, mod_structs::Mod};

use crate::mod_::{Mod as RealMod, ModVersion};

impl From<Mod> for RealMod {
    fn from(val: Mod) -> Self {
        Self {
            id: val.id.to_string(),
            game_id: Some(val.game_id),
            versions: val
                .latest_files
                .iter()
                .map(|v| v.clone().into())
                .collect::<Vec<ModVersion>>(),
            name: val.name,
            source: Sources::CurseForge.id(),
        }
    }
}

impl From<File> for ModVersion {
    fn from(val: File) -> Self {
        let hash = val.hashes.first().map(|v| v.value.clone());

        Self {
            id: val.id.to_string(),
            file_name: Some(val.file_name),
            hash,
            url: None,
            name: Some(val.display_name),
            size: Some(val.file_length.to_string()),
        }
    }
}
