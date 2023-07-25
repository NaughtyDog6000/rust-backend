use std::arch::x86_64::_XCR_XFEATURE_ENABLED_MASK;

use axum::{Extension, Json, Router, routing::get, http::StatusCode, response::{IntoResponse, Response}};
use log::{warn, info};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{pool, PgPool};

use crate::{utils::check_token, structs::User};




pub fn router() -> Router {
    Router::new().route("/test_token",
    get(|| async {"This does NOT support get requests"}).post(test_token)
    )
}

#[derive(Serialize, Deserialize)]
pub struct TokenRequestParams {
    pub token: String
}


pub async fn test_token(
    Extension(pool): Extension<PgPool>,
    Json(request): Json<TokenRequestParams>
) -> (StatusCode, Json<Value>) {

    let result = check_token(&pool, request.token).await;
    if result.is_err()
    {
        return (StatusCode::BAD_REQUEST, Json(json!("bad token")));
    }
    let user = result.unwrap();
    info!("user: {}\ntested a token", user.username);

    (StatusCode::OK, Json(json!(user.username)))
}