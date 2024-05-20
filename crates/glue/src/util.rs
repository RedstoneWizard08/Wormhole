// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use axum::{body::Body, extract::Request};
use http_body_util::BodyExt;
use hyper::{
    body::{Bytes, Incoming},
    Response,
};

pub fn is_debug() -> bool {
    cfg_if! {
        if #[cfg(debug_assertions)] {
            true
        } else {
            false
        }
    }
}

pub async fn req_to_bytes(mut body: Request<Body>) -> Bytes {
    let mut bytes = Vec::new();

    while let Some(Ok(frame)) = body.frame().await {
        if let Some(chunk) = frame.data_ref() {
            for byte in chunk {
                bytes.push(*byte);
            }
        }
    }

    Bytes::from_iter(bytes)
}

pub async fn res_to_bytes(mut body: Response<Incoming>) -> Bytes {
    let mut bytes = Vec::new();

    while let Some(Ok(frame)) = body.frame().await {
        if let Some(chunk) = frame.data_ref() {
            for byte in chunk {
                bytes.push(*byte);
            }
        }
    }

    Bytes::from_iter(bytes)
}

pub fn scheme_port(scheme: &str) -> u16 {
    if scheme.starts_with("https") {
        443
    } else {
        80
    }
}
