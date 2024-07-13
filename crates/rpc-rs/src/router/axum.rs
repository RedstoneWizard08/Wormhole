//! The [`axum`] support module.

use std::{future::Future, pin::Pin, sync::Arc};

use crate::{module::module::Module, proc::GenericProcedure, util::TripleS};
use axum::{
    body::Body,
    extract::Request,
    handler::Handler,
    response::Response,
    routing::{any, post},
};
use http_body_util::BodyExt;

use super::{router::Router, Method};

impl<Cx: TripleS + Clone> Router<Cx> {
    /// Convert this router into an [`axum::Router`], consuming it.
    pub fn axum(self, state: Cx) -> axum::Router<Cx> {
        let mut router = axum::Router::new().with_state(state);

        for (name, module) in self.modules {
            router = router.route(&format!("/{}", name), any(module));
        }

        for (name, handler) in self.invokers {
            router = router.route(&format!("/_invoke/{}", name), post(ProcWrapper(handler)));
        }

        router
    }
}

impl From<axum::http::Method> for Method {
    fn from(value: axum::http::Method) -> Self {
        match value {
            axum::http::Method::POST => Self::Read,
            axum::http::Method::PUT => Self::Create,
            axum::http::Method::PATCH => Self::Update,
            axum::http::Method::DELETE => Self::Delete,
            _ => Self::Error,
        }
    }
}

impl<Cx: TripleS + Clone> Handler<((), Request), Cx> for Module<Cx> {
    type Future = Pin<Box<dyn Future<Output = Response> + Send + 'static>>;

    fn call(self, req: Request, state: Cx) -> Self::Future {
        Box::pin(async move {
            let method = req.method().clone();
            let mut stream = req.into_body().into_data_stream();
            let mut data = Vec::new();

            while let Some(Ok(frame)) = stream.frame().await {
                if frame.is_data() {
                    let bytes = frame.into_data().unwrap();

                    data.extend(bytes);
                }
            }

            let data = String::from_utf8(data).unwrap_or_default();

            match self.exec(state, Method::from(method), data).await {
                Ok(data) => Response::new(Body::new(data)),
                Err(err) => Response::builder()
                    .status(500)
                    .body(Body::new(err.to_string()))
                    .unwrap(),
            }
        })
    }
}

#[derive(Clone)]
struct ProcWrapper<Cx: TripleS + Clone>(Arc<Box<dyn GenericProcedure<Cx> + TripleS>>);

impl<Cx: TripleS + Clone> Handler<((), Request), Cx> for ProcWrapper<Cx> {
    type Future = Pin<Box<dyn Future<Output = Response> + Send + 'static>>;

    fn call(self, req: Request, state: Cx) -> Self::Future {
        Box::pin(async move {
            let mut stream = req.into_body().into_data_stream();
            let mut data = Vec::new();

            while let Some(Ok(frame)) = stream.frame().await {
                if frame.is_data() {
                    let bytes = frame.into_data().unwrap();

                    data.extend(bytes);
                }
            }

            let data = String::from_utf8(data).unwrap_or_default();

            match self.0.run(state, data).await {
                Ok(data) => Response::new(Body::new(data)),
                Err(err) => Response::builder()
                    .status(500)
                    .body(Body::new(err.to_string()))
                    .unwrap(),
            }
        })
    }
}
