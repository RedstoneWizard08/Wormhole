// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use std::net::SocketAddr;

use axum::{
    body::Body,
    debug_handler,
    extract::{ConnectInfo, Request},
    response::IntoResponse,
    Extension,
};
use axumite::upgrade::WebSocketUpgrade;
use hyper::{
    header::{SEC_WEBSOCKET_PROTOCOL, USER_AGENT},
    HeaderMap,
};
use tungstenite::http::HeaderValue;

use crate::{state::ProxyState, ws::handler::handle_websocket};

#[debug_handler]
pub async fn websocket_handler(
    Extension(state): Extension<ProxyState>,
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request<Body>,
) -> impl IntoResponse {
    let user_agent = headers
        .get(USER_AGENT)
        .map(|v| v.to_str().unwrap())
        .unwrap_or("Unknown browser");

    debug!("{} connected. (UA: {})", addr, user_agent);

    let cloned_state = state.clone();

    let mut res = ws
        .on_upgrade(move |socket| handle_websocket(cloned_state, req.uri().clone(), socket, addr));

    if let Some(proto) = state.framework.get_subprotocol() {
        res.headers_mut().insert(
            SEC_WEBSOCKET_PROTOCOL,
            HeaderValue::from_str(proto).unwrap(),
        );
    }

    res
}
