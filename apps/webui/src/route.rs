// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use crate::state::AppState;
use anyhow::{anyhow, Result};
use axum::{
    body::Body,
    extract::{Request, State},
    http::header::CONTENT_TYPE,
};
use http_body_util::BodyExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use whcore::state::TState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvokeInput {
    pub cmd: String,
    pub data: Value,
}

#[axum::debug_handler]
pub async fn route_handler(State(state): State<AppState>, req: Request<Body>) -> Result<String> {
    let headers = req.headers().clone();
    let (_, body) = req.into_parts();
    let body = body.collect().await?.to_bytes().to_vec();
    let body = String::from_utf8(body)?;

    if !headers[CONTENT_TYPE].to_str()?.contains("application/json") {
        return Err(anyhow!("Request body must be JSON!"));
    }

    let data = serde_json::from_str::<InvokeInput>(&body)?;

    Ok(wormhole_gui::handle_serde_call(
        data.cmd,
        serde_json::to_string(&data.data)?,
        TState(&state.pool),
    )
    .await)
}
