//! The [`ModuleBuilder`] module.

use crate::{proc::GenericProcedure, util::TripleS};

use super::module::Module;

/// A builder for a [`Module`].
pub struct ModuleBuilder<Cx: TripleS + Clone> {
    /// The handler for [`rpc_rs::Method::Create`] calls.
    pub(crate) create: Option<Box<dyn GenericProcedure<Cx> + TripleS>>,

    /// The handler for [`rpc_rs::Method::Read`] calls.
    pub(crate) read: Option<Box<dyn GenericProcedure<Cx> + TripleS>>,

    /// The handler for [`rpc_rs::Method::Update`] calls.
    pub(crate) update: Option<Box<dyn GenericProcedure<Cx> + TripleS>>,

    /// The handler for [`rpc_rs::Method::Delete`] calls.
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
    /// Attach a handler for [`rpc_rs::Method::Create`].
    pub fn create<Proc: GenericProcedure<Cx> + TripleS>(mut self, proc: Proc) -> Self {
        self.create = Some(Box::new(proc));
        self
    }

    /// Attach a handler for [`rpc_rs::Method::Read`].
    pub fn read<Proc: GenericProcedure<Cx> + TripleS>(mut self, proc: Proc) -> Self {
        self.read = Some(Box::new(proc));
        self
    }

    /// Attach a handler for [`rpc_rs::Method::Update`].
    pub fn update<Proc: GenericProcedure<Cx> + TripleS>(mut self, proc: Proc) -> Self {
        self.update = Some(Box::new(proc));
        self
    }

    /// Attach a handler for [`rpc_rs::Method::Delete`].
    pub fn delete<Proc: GenericProcedure<Cx> + TripleS>(mut self, proc: Proc) -> Self {
        self.delete = Some(Box::new(proc));
        self
    }

    /// Build this [`ModuleBuilder`], converting it into a [`Module`].
    pub fn build(self) -> Module<Cx> {
        Module::new(self)
    }
}
