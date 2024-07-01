use axum::http::StatusCode;
use axum::Json;
use crate::context::response_helper::JsonErrorResponse;

pub fn exists_user_error(err: anyhow::Error) -> (StatusCode, Json<JsonErrorResponse>) {
    if err.to_string() == *"User exists" {
        let json = JsonErrorResponse::new(
            "invalid_request".to_string(),
            vec!["A user with this data already exists".to_string()],
        );
        (StatusCode::BAD_REQUEST, Json(json))
    } else {
        let json = JsonErrorResponse::new(
            "server_error".to_string(),
            vec!["INTERNAL SERVER ERROR".to_string()],
        );
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json))
    }
}
