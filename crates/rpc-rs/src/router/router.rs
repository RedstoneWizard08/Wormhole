//! The router.

use std::collections::HashMap;

use specta::{
    ts::{BigIntExportBehavior, ExportConfig},
    TypeMap,
};

use crate::{module::module::Module, util::TripleS};

/// A router.
#[allow(dead_code)] // It's not actually dead code, generics just make it seem so.
#[derive(Clone, Default)]
pub struct Router<Cx: TripleS + Clone> {
    /// A map of IDs to modules.
    pub(crate) modules: HashMap<String, Module<Cx>>,

    /// The [`TypeMap`] for this router.
    pub(crate) type_map: TypeMap,

    /// The [`ExportConfig`] for this router.
    pub(crate) export_cfg: ExportConfig,
}

impl<Cx: TripleS + Clone> Router<Cx> {
    /// Get the default [`ExportConfig`].
    pub(crate) fn export_config() -> ExportConfig {
        ExportConfig::default().bigint(BigIntExportBehavior::BigInt)
    }

    /// Create a new [`Router`].
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            type_map: TypeMap::default(),
            export_cfg: Self::export_config(),
        }
    }

    /// Mount a [`Module`] with an ID of anything that implements [`AsRef<str>`].
    pub fn mount(mut self, name: impl AsRef<str>, module: Module<Cx>) -> Self {
        self.modules.insert(name.as_ref().to_string(), module);
        self
    }
}
