//! Extension traits

use anyhow::Result;
use tauri::{Builder, Runtime};
use api::tauri::TauriPlugin;
use crate::default_plugins;

/// An extension to Tauri's [`Builder`] type to automatically register all plugins.
pub trait PluginRegister {
    /// Automatically register all plugins.
    fn add_plugins(self) -> Result<Self> where Self: Sized;
}

impl<R: Runtime> PluginRegister for Builder<R> {
    fn add_plugins(self) -> Result<Self> {
        let mut me = self;
        
        for it in default_plugins() {
            me = me.plugin(TauriPlugin::new_boxed(it)?);
        }
        
        Ok(me)
    }
}
