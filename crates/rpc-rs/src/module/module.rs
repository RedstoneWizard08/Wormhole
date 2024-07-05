use std::sync::Arc;

use serde_json::Error;

use crate::{router::Method, proc::{wrap, GenericProcedure}, util::TripleS};

use super::builder::ModuleBuilder;

#[derive(Clone)]
pub struct Module<Cx: TripleS + Clone> {
    create: Arc<Box<dyn GenericProcedure<Cx> + TripleS>>,
    read: Arc<Box<dyn GenericProcedure<Cx> + TripleS>>,
    update: Arc<Box<dyn GenericProcedure<Cx> + TripleS>>,
    delete: Arc<Box<dyn GenericProcedure<Cx> + TripleS>>,
}

impl<Cx: TripleS + Clone> Module<Cx> {
    pub(crate) fn new(builder: ModuleBuilder<Cx>) -> Self {
        Self {
            create: Arc::new(builder.create.unwrap_or(Box::new(wrap(Self::error_responder)))),
            read: Arc::new(builder.read.unwrap_or(Box::new(wrap(Self::error_responder)))),
            update: Arc::new(builder.update.unwrap_or(Box::new(wrap(Self::error_responder)))),
            delete: Arc::new(builder.delete.unwrap_or(Box::new(wrap(Self::error_responder)))),
        }
    }

    pub(crate) async fn error_responder(_cx: Cx, _: ()) -> String {
        format!("404 | Route not found")
    }

    pub async fn exec(&self, cx: Cx, method: Method, data: String) -> Result<String, Error> {
        match method {
            Method::Create => self.create.run(cx, data).await,
            Method::Read => self.read.run(cx, data).await,
            Method::Update => self.update.run(cx, data).await,
            Method::Delete => self.delete.run(cx, data).await,
            Method::Error => Ok(Self::error_responder(cx, ()).await),
        }
    }

    pub fn builder() -> ModuleBuilder<Cx> {
        ModuleBuilder::default()
    }
}
