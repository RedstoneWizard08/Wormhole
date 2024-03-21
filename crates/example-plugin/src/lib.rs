use api::res::PluginResult;

#[no_link]
extern crate api;

#[no_mangle]
pub fn wormhole_plugin_init() -> PluginResult {
    std::println!("Hello, world!");

    PluginResult::Ok
}
