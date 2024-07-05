use crate::{proc::GenericProcedure, util::TripleS};

use super::module::Module;

pub struct ModuleBuilder<Cx: TripleS + Clone> {
    pub(crate) create: Option<Box<dyn GenericProcedure<Cx> + TripleS>>,
    pub(crate) read: Option<Box<dyn GenericProcedure<Cx> + TripleS>>,
    pub(crate) update: Option<Box<dyn GenericProcedure<Cx> + TripleS>>,
    pub(crate) delete: Option<Box<dyn GenericProcedure<Cx> + TripleS>>,
}

impl<Cx: TripleS + Clone> Default for ModuleBuilder<Cx> {
    fn default() -> Self {
        Self {
            create: None,
            read: None,
            update: None,
            delete: None,
        }
    }
}

impl<Cx: TripleS + Clone> ModuleBuilder<Cx> {
    pub fn create<Proc: GenericProcedure<Cx> + TripleS>(mut self, proc: Proc) -> Self {
        self.create = Some(Box::new(proc));
        self
    }

    pub fn read<Proc: GenericProcedure<Cx> + TripleS>(mut self, proc: Proc) -> Self {
        self.read = Some(Box::new(proc));
        self
    }

    pub fn update<Proc: GenericProcedure<Cx> + TripleS>(mut self, proc: Proc) -> Self {
        self.update = Some(Box::new(proc));
        self
    }

    pub fn delete<Proc: GenericProcedure<Cx> + TripleS>(mut self, proc: Proc) -> Self {
        self.delete = Some(Box::new(proc));
        self
    }

    pub fn build(self) -> Module<Cx> {
        Module::new(self)
    }
}
