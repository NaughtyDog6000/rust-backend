use axum::{
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Extension, Json, Router,
};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{pool, PgPool};

use crate::{
    structs::User,
    utils::{check_token, get_user},
};

pub async fn test_token(
    Extension(pool): Extension<PgPool>,
    headers: HeaderMap,
) -> (StatusCode, Json<Value>) {
    let auth_token = headers.get("auth");
    if auth_token.is_none() {
        return (
            StatusCode::IM_A_TEAPOT,
            Json(json!({
                "response": "token not present you melon"
            })),
        );
    }
    let auth_token = auth_token.unwrap().to_str().unwrap().to_owned();

    let result = get_user(&pool, None, None, Some(auth_token)).await;
    if result.is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"ERROR" : "bad token"})),
        );
    }
    let user = result.unwrap();
    info!("user: {}\ntested a token", user.username);

    (StatusCode::OK, Json(json!(user.username)))
}
