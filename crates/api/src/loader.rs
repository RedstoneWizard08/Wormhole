#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Type)]
pub enum ModLoader {
    Minecraft(mcmeta::cmd::modded::ModLoader),
    Vanilla(Option<String>),
}
