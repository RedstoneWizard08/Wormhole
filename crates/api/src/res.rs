use std::mem::transmute;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum PluginResult {
    Ok = 0,
    Error = 1,

    #[default]
    Unknown = -1,
}

impl From<i32> for PluginResult {
    fn from(value: i32) -> Self {
        unsafe { transmute(value) }
    }
}
