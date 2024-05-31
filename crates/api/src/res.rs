//! The result of a plugin call.

use std::mem::transmute;

/// The result of a plugin call.
/// This was used back when I tried to implement WASI plugins
/// for the first time, but it's no longer used. I'm keeping it
/// so I have less work to do in the future.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum PluginResult {
    /// The plugin call was successful.
    Ok = 0,

    /// The plugin call failed.
    Error = 1,

    /// The plugin call's result was unknown.
    #[default]
    Unknown = -1,
}

impl From<i32> for PluginResult {
    fn from(value: i32) -> Self {
        unsafe { transmute(value) }
    }
}
