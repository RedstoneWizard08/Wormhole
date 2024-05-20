// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use std::sync::{Arc, Mutex};

use crate::midlog_log;
use axum::{body::Body, http::Request, middleware::Next, response::Response};
use chrono::Utc;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref FILTERS: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));
}

pub async fn logging_middleware(req: Request<Body>, next: Next) -> Response {
    let time_start = Utc::now().time();
    let method = &req.method().clone();
    let uri = &req.uri().clone();
    let res = next.run(req).await;
    let now = Utc::now().time();
    let elapsed = now - time_start;
    let path = uri.path_and_query().unwrap().as_str();

    for item in FILTERS.lock().unwrap().iter().cloned() {
        if path.contains(item) {
            return res;
        }
    }

    midlog_log!(
        method.as_str(),
        path,
        res.status(),
        elapsed.num_milliseconds()
    );

    res
}
