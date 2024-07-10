//! The [`Module`] module. module.

use std::sync::Arc;

use serde_json::Error;

use crate::{
    proc::{wrap, GenericProcedure},
    router::Method,
    util::TripleS,
};

use super::builder::ModuleBuilder;

/// A module.
#[derive(Clone)]
pub struct Module<Cx: TripleS + Clone> {
    /// The handler for [`rpc_rs::Method::Create`] calls.
    pub(crate) create: Arc<Box<dyn GenericProcedure<Cx> + TripleS>>,

    /// The handler for [`rpc_rs::Method::Read`] calls.
    pub(crate) read: Arc<Box<dyn GenericProcedure<Cx> + TripleS>>,

    /// The handler for [`rpc_rs::Method::Update`] calls.
    pub(crate) update: Arc<Box<dyn GenericProcedure<Cx> + TripleS>>,

    /// The handler for [`rpc_rs::Method::Delete`] calls.
    pub(crate) delete: Arc<Box<dyn GenericProcedure<Cx> + TripleS>>,
}

impl<Cx: TripleS + Clone> Module<Cx> {
    /// Create a new [`Module`] from a [`ModuleBuilder`].
    pub(crate) fn new(builder: ModuleBuilder<Cx>) -> Self {
        Self {
            create: Arc::new(
                builder
                    .create
                    .unwrap_or(Box::new(wrap(Self::error_responder))),
            ),
            read: Arc::new(
                builder
                    .read
                    .unwrap_or(Box::new(wrap(Self::error_responder))),
            ),
            update: Arc::new(
                builder
                    .update
                    .unwrap_or(Box::new(wrap(Self::error_responder))),
            ),
            delete: Arc::new(
                builder
                    .delete
                    .unwrap_or(Box::new(wrap(Self::error_responder))),
            ),
        }
    }

    /// The default handler for any unhandled call.
    pub(crate) async fn error_responder(_cx: Cx, _: ()) -> String {
        format!("404 | Route not found")
    }

    /// Try to have this module handle a request.
    pub async fn exec(&self, cx: Cx, method: Method, data: String) -> Result<String, Error> {
        match method {
            Method::Create => self.create.run(cx, data).await,
            Method::Read => self.read.run(cx, data).await,
            Method::Update => self.update.run(cx, data).await,
            Method::Delete => self.delete.run(cx, data).await,
            Method::Error => Ok(Self::error_responder(cx, ()).await),
        }
    }

    /// Create a new [`ModuleBuilder`].
    pub fn builder() -> ModuleBuilder<Cx> {
        ModuleBuilder::default()
    }
}
