use api::PluginResult;

#[no_link]
extern crate wormhole_api as api;

#[no_mangle]
pub fn wormhole_plugin_init() -> PluginResult {
    std::println!("Hello, world!");

    PluginResult::Ok
}
