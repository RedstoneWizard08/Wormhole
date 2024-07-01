//! The router module.

use std::sync::Arc;

use axum::{middleware::from_fn, routing::get, Router};
use commands::router::build_router;
use data::{get_client, get_or_init_client};
use glue::{glue::Glue, util::is_debug};
use midlog::logging_middleware;

use crate::code::{self, is_openvscode_server};

/// A builder for the router.
#[derive(Debug, Clone)]
pub struct RouterBuilder {
    router: Router,
}

impl RouterBuilder {
    /// Create a new router builder.
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }

    /// Register the glue.
    pub fn glue(self, glue: Glue) -> Self {
        let mut new = Self::new();
        new.router = glue.register(self.router, is_debug());
        new
    }

    /// Add the logging middleware.
    pub fn log(self) -> Self {
        let mut new = Self::new();
        new.router = self.router.layer(from_fn(logging_middleware));
        new
    }

    /// Add the routes.
    pub async fn routes(self) -> Self {
        let mut new = Self::new();
        let rspc = build_router();

        get_or_init_client().await.unwrap();

        let endpoint = rspc_axum::endpoint(Arc::new(rspc), get_client);

        // TODO: Axum RSPC
        new.router = new.router.nest("/rspc", endpoint);

        if is_openvscode_server().unwrap() {
            new.router = new.router.route("/__open-in-editor", get(code::handler));
        }

        new
    }

    /// Build the router.
    pub fn build(self) -> Router {
        self.router
    }
}

impl Default for RouterBuilder {
    fn default() -> Self {
        Self::new()
    }
}
