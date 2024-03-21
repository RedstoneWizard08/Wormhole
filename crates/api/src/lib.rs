#[cfg(not(target_arch = "wasm32"))]
pub mod load;

#[cfg(not(target_arch = "wasm32"))]
pub use load::*;

#[macro_use]
extern crate lazy_static;
pub extern crate whcore;

pub mod plugin;
pub mod register;
pub mod res;

/// The name of the plugin initializer function.
pub const INIT_FN_NAME: &str = "wormhole_plugin_init";

/// A plugin initializer.
/// Since `anyhow`'s `Result` type doesn't support `#[repr(C)]`,
/// this is a function with no arguments that returns a `PluginResult`.
pub type PluginInitializer = unsafe extern "C" fn() -> i32;
