//! The router module.

use axum::{
    middleware::from_fn,
    routing::{get, post},
    Router,
};
use glue::{glue::Glue, util::is_debug};
use midlog::logging_middleware;

use crate::{
    code::{self, is_openvscode_server},
    route::route_handler,
    state::AppState,
    ws::websocket_handler,
};

/// A builder for the router.
#[derive(Debug, Clone)]
pub struct RouterBuilder {
    router: Router<AppState>,
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
    pub fn routes(self) -> Self {
        let mut new = Self::new();

        new.router = self.router.route("/_tauri/invoke", post(route_handler));
        new.router = new.router.route("/_tauri/events", get(websocket_handler));

        if is_openvscode_server().unwrap() {
            new.router = new.router.route("/__open-in-editor", get(code::handler));
        }

        new
    }

    /// Build the router.
    pub fn build(self, state: AppState) -> Router {
        self.router.with_state(state)
    }
}

impl Default for RouterBuilder {
    fn default() -> Self {
        Self::new()
    }
}
