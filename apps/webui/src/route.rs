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
    ).await)
}
