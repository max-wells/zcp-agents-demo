use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct OrderApiError {
    pub error: String,
}

#[derive(Debug, thiserror::Error)]
pub enum OrderError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("{0}")]
    BadRequest(String),
}

impl IntoResponse for OrderError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            OrderError::Database(ref e) => {
                let error_message = e.to_string();
                tracing::error!("database error: {error_message}");
                (StatusCode::INTERNAL_SERVER_ERROR, error_message)
            }
            OrderError::BadRequest(ref msg) => (StatusCode::BAD_REQUEST, msg.clone()),
        };

        (status, Json(OrderApiError { error: error_message })).into_response()
    }
}
