//! The router module.

use std::sync::Arc;

use axum::{middleware::from_fn, routing::get, Router};
use commands::router::build_router;
use data::{get_or_init_client, prisma::PrismaClient};
use glue::{glue::Glue, util::is_debug};
use midlog::logging_middleware;

use crate::code::{self, is_openvscode_server};

/// A builder for the router.
#[derive(Debug, Clone)]
pub struct RouterBuilder {
    router: Router<Arc<PrismaClient>>,
}

impl RouterBuilder {
    /// Create a new router builder.
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }

    /// Register the glue.
    pub fn glue(mut self, glue: Glue) -> Self {
        self.router = glue.register(self.router, is_debug());
        self
    }

    /// Add the logging middleware.
    pub fn log(mut self) -> Self {
        self.router = self.router.layer(from_fn(logging_middleware));
        self
    }

    /// Add the routes.
    pub async fn routes(mut self) -> Self {
        let db = get_or_init_client().await.unwrap();

        // TODO: Axum RSPC
        self.router = self.router.nest("/rpc", build_router().axum(db));

        if is_openvscode_server().unwrap() {
            self.router = self.router.route("/__open-in-editor", get(code::handler));
        }

        self
    }

    /// Build the router.
    pub async fn build(self) -> Router {
        self.router.with_state(get_or_init_client().await.unwrap())
    }
}
