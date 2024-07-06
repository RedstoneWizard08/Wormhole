use std::collections::HashMap;

use specta::{ts::{BigIntExportBehavior, ExportConfig}, TypeMap};

use crate::{module::module::Module, util::TripleS};

#[allow(dead_code)]
#[derive(Clone, Default)]
pub struct Router<Cx: TripleS + Clone> {
    pub(crate) modules: HashMap<String, Module<Cx>>,
    pub(crate) type_map: TypeMap,
    pub(crate) export_cfg: ExportConfig,
}

impl<Cx: TripleS + Clone> Router<Cx> {
    pub(crate) fn export_config() -> ExportConfig {
        ExportConfig::default().bigint(BigIntExportBehavior::BigInt)
    }
    
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            type_map: TypeMap::default(),
            export_cfg: Self::export_config(),
        }
    }

    pub fn mount(mut self, name: impl AsRef<str>, module: Module<Cx>) -> Self {
        self.modules.insert(name.as_ref().to_string(), module);
        self
    }
}
