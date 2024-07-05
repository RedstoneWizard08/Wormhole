use std::collections::HashMap;

use crate::{module::module::Module, util::TripleS};

#[derive(Clone, Default)]
pub struct Router<Cx: TripleS + Clone> {
    pub(crate) modules: HashMap<String, Module<Cx>>,
}

impl<Cx: TripleS + Clone> Router<Cx> {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn mount(mut self, name: impl AsRef<str>, module: Module<Cx>) -> Self {
        self.modules.insert(name.as_ref().to_string(), module);
        self
    }
}
