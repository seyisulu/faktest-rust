use axum::{Router, routing::post, Json, extract::State};
use crate::error::AppError;

pub fn routes() -> Router {
    Router::new()
        .route("/login", post(login))
}

async fn login() -> Result<Json<serde_json::Value>, AppError> {
    // TODO: Implement full Argon2 + JWT
    Ok(Json(serde_json::json!({"message": "Login endpoint placeholder - Argon2 auth coming"})))
}
