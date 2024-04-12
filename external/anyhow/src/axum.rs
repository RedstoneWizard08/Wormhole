use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::Error;

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::new(std::format!("{:?}", self)))
            .unwrap()
    }
}
