//TODO: make an http server to listen to requests being set from the front end

use crate::prelude::*;
use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::post};
use serde_json::json;

#[derive(Debug)]
enum ApiError {
    NotFound,
    InvalidInput(String),
    InternalError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Data not found".to_string()),
            ApiError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

async fn handler() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "message": "server is running"
    }))

}

async fn create_app() -> Router {
    Router::new().route("/api/coordinates", post(handler))
}

#[tokio::main]
async fn run_server() {
    let app = create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind tcp listener");

    println!("server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .expect("failed to start server");
}
