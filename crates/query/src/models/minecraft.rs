use super::project::ProjectKind;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub enum ModLoader {
    Forge,
    NeoForge,
    Fabric,
    Quilt,
    Vanilla,
}

impl ModLoader {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Forge => "forge",
            Self::NeoForge => "neoforge",
            Self::Fabric => "fabric",
            Self::Quilt => "quilt",
            Self::Vanilla => "vanilla",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct MinecraftInput {
    pub kind: ProjectKind,
    pub loader: ModLoader,
    pub mc_version: String,
}
