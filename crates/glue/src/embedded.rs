// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use axum::{body::Body, debug_handler, extract::Request, response::Response, Extension};
use hyper::{header::CONTENT_TYPE, StatusCode};
use include_dir::Dir;

#[debug_handler]
pub async fn handle_embedded(
    Extension(dir): Extension<Dir<'static>>,
    req: Request<Body>,
) -> Response<Body> {
    let path = req.uri().path();
    let index_path = format!("{}index.html", path);

    let path = if path.ends_with('/') {
        index_path.as_str()
    } else {
        path
    };

    let path = path.trim_start_matches('/');

    // Try the file
    if let Some(file) = dir.get_file(path) {
        let mime = mime_guess::from_path(path).first().unwrap();

        return Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, mime.to_string())
            .body(Body::from(file.contents().to_vec()))
            .unwrap();
    }

    // Try a fallback page
    if let Some(file) = dir.get_file("fallback.html") {
        return Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, "text/html")
            .body(Body::from(file.contents().to_vec()))
            .unwrap();
    }

    // Try a 404 page
    if let Some(file) = dir.get_file("404.html") {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(CONTENT_TYPE, "text/html")
            .body(Body::from(file.contents().to_vec()))
            .unwrap();
    }

    // Maybe it's an SPA?
    if let Some(file) = dir.get_file("index.html") {
        return Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, "text/html")
            .body(Body::from(file.contents().to_vec()))
            .unwrap();
    }

    // 404 not found in plaintext
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(
            "Cannot find the file specified!".as_bytes().to_vec(),
        ))
        .unwrap()
}
