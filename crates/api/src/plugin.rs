use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plugin {
    pub id: i32,
    pub name: String,
    pub display: String,
    pub icon: String,
    pub banner: String,
    pub caps: PluginCaps,
    pub settings: HashMap<String, ()>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PluginCaps {
    pub multiple_instances: bool,
    pub mods: bool,
}
