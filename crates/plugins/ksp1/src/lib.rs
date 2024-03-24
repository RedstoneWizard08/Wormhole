use api::{plugin::Plugin, register::register_plugin, res::PluginResult};
use base64::{engine::general_purpose::STANDARD, Engine};
use query::{source::Source, spacedock::SpaceDock};

#[no_link]
extern crate api;

pub const ICON_BYTES: &[u8] = include_bytes!("../assets/icon.png");
pub const BANNER_BYTES: &[u8] = include_bytes!("../assets/banner.png");

pub struct KSP1Plugin;

impl Plugin for KSP1Plugin {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn id(&self) -> String {
        "Kerbal Space Program".into()
    }

    fn game(&self) -> i32 {
        3102
    }

    fn icon(&self) -> String {
        format!("data:image/png;base64,{}", STANDARD.encode(ICON_BYTES))
    }

    fn banner(&self) -> String {
        format!("data:image/png;base64,{}", STANDARD.encode(BANNER_BYTES))
    }

    fn display(&self) -> String {
        "Kerbal Space Program".into()
    }

    fn fallback(&self) -> Option<&str> {
        Some("GameData")
    }

    fn resolvers(&self) -> Vec<Box<dyn Source>> {
        vec![Box::new(SpaceDock::new())]
    }
}

#[no_mangle]
pub fn wormhole_plugin_init() -> PluginResult {
    println!("[KSP 1] Registering support for Kerbal Space Program...");

    register_plugin(Box::new(KSP1Plugin::new()));

    println!("[KSP 1] Registered!");

    PluginResult::Ok
}
