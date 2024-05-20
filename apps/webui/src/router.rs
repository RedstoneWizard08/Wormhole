// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use axum::{middleware::from_fn, routing::post, Router};
use glue::{glue::Glue, util::is_debug};
use midlog::logging_middleware;

use crate::{route::route_handler, state::AppState};

#[derive(Debug, Clone)]
pub struct RouterBuilder {
    router: Router<AppState>,
}

impl RouterBuilder {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }

    pub fn glue(self, glue: Glue) -> Self {
        let mut new = Self::new();
        new.router = glue.register(self.router, is_debug());
        new
    }

    pub fn log(self) -> Self {
        let mut new = Self::new();
        new.router = self.router.layer(from_fn(logging_middleware));
        new
    }

    pub fn routes(self) -> Self {
        let mut new = Self::new();
        new.router = self.router.route("/_tauri/invoke", post(route_handler));
        new
    }

    pub fn build(self, state: AppState) -> Router {
        self.router.with_state(state)
    }
}

impl Default for RouterBuilder {
    fn default() -> Self {
        Self::new()
    }
}
