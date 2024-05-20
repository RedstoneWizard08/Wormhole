// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use axum::{routing::get, Extension, Router};
use include_dir::Dir;

use crate::{
    embedded::handle_embedded, framework::Framework, handler::fallback_handler, state::ProxyState,
    ws::route::websocket_handler,
};

/// Register the proxy handler.
/// Accepts a base and a router.
/// The base must be in the format "http(s)://[ip or domain]:[optional port]"
/// with NO trailing slash (or it will break).
pub fn register_proxy<T>(base: String, router: Router<T>, framework: Option<Framework>) -> Router<T>
where
    T: Clone + Send + Sync + 'static,
{
    let state = ProxyState::new(base, framework.unwrap_or(Framework::None));
    let mut router = router;

    if let Some(framework) = framework {
        router = router.route(framework.get_hmr_path(), get(websocket_handler));
    }

    router.fallback(fallback_handler).layer(Extension(state))
}

pub fn register_embedded<T>(dir: Dir<'static>, router: Router<T>) -> Router<T>
where
    T: Clone + Send + Sync + 'static,
{
    router.fallback(handle_embedded).layer(Extension(dir))
}
