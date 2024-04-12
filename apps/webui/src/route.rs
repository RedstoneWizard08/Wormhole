use std::{
    collections::HashMap,
    sync::{
        mpsc::{channel, Sender},
        Arc, Mutex,
    },
};

use crate::state::AppState;
use anyhow::{anyhow, Result};
use axum::{
    body::Body,
    extract::{Query, Request, State},
    http::header::CONTENT_TYPE,
};
use http_body_util::BodyExt;
use lazy_static::lazy_static;
use tauri::{InvokePayload, InvokeResponse, Manager};

lazy_static! {
    pub static ref CHANNELS: Arc<Mutex<HashMap<usize, Sender<InvokeResponse>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

#[axum::debug_handler]
pub async fn route_handler(
    State(state): State<AppState>,
    Query(window): Query<Option<String>>,
    req: Request<Body>,
) -> Result<String> {
    let headers = req.headers().clone();
    let (_, body) = req.into_parts();
    let body = body.collect().await?.to_bytes().to_vec();
    let body = String::from_utf8(body)?;

    if !headers[CONTENT_TYPE].to_str()?.contains("application/json") {
        return Err(anyhow!("Request body must be JSON!"));
    }

    let data = serde_json::from_str::<InvokePayload>(&body)?;

    let win = if let Some(window) = window {
        state.app.get_window(&window)
    } else {
        state.app.get_focused_window()
    };

    if let Some(win) = win {
        let id = data.callback.0;
        let (tx, rx) = channel();

        CHANNELS.lock().unwrap().insert(id, tx);
        win.on_message(data)?;

        if let Ok(data) = rx.recv() {
            Ok(serde_json::to_string(&match data.into_result() {
                Ok(val) => val,
                Err(err) => err,
            })?)
        } else {
            Err(anyhow!("Didn't get a response!"))
        }
    } else {
        Err(anyhow!("Cannot find a window!"))
    }
}
