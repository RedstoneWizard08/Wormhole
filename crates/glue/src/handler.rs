// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use crate::{
    query::parse_query,
    runner::CLIENT_READY,
    state::ProxyState,
    util::{req_to_bytes, res_to_bytes},
};
use axum::{body::Body, debug_handler, extract::Request, response::Response, Extension};
use hyper::{
    header::{CONTENT_LENGTH, CONTENT_TYPE, TRANSFER_ENCODING},
    StatusCode,
};

#[debug_handler]
pub async fn fallback_handler(
    Extension(state): Extension<ProxyState>,
    req: Request<Body>,
) -> Response<Body> {
    unsafe {
        if !CLIENT_READY {
            return Response::builder()
                .status(StatusCode::SERVICE_UNAVAILABLE)
                .header(CONTENT_TYPE, "text/plain")
                .body(Body::from("Frontend is starting..."))
                .unwrap();
        }
    }

    let method = req.method().clone();
    let path = req.uri().clone();
    let path = path.path();
    let uri = req.uri().clone();
    let headers = req.headers().clone();

    let res = state
        .request(
            method,
            path,
            uri.query().map(parse_query),
            Some(req_to_bytes(req).await),
            Some(headers),
        )
        .await
        .unwrap();

    let mut builder = Response::builder().status(res.status());

    for key in res.headers().keys() {
        if *key == CONTENT_LENGTH || *key == TRANSFER_ENCODING {
            continue;
        }

        builder = builder.header(key, res.headers().get(key).unwrap());
    }

    builder.body(Body::from(res_to_bytes(res).await)).unwrap()
}
