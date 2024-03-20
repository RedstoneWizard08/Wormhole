#[cfg(feature = "loading")]
pub mod load;

#[cfg(feature = "loading")]
pub use load::*;

pub mod res;
pub use res::PluginResult;

/// The name of the plugin initializer function.
pub const INIT_FN_NAME: &str = "wormhole_plugin_init";

/// A plugin initializer.
/// Since `anyhow`'s `Result` type doesn't support `#[repr(C)]`,
/// this is a function with no arguments that returns a `PluginResult`.
pub type PluginInitializer = unsafe extern fn() -> i32;
