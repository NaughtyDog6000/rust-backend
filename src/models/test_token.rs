use axum::{Extension, Json, Router, routing::get, http::{StatusCode, HeaderMap}, response::{IntoResponse, Response}};
use log::{warn, info};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{pool, PgPool};

use crate::{utils::{check_token, get_user}, structs::User};




pub fn router() -> Router {
    Router::new().route("/test_token",
    get(|| async {"This does NOT support get requests"}).post(test_token)
    )
}




pub async fn test_token(
    Extension(pool): Extension<PgPool>,
    headers: HeaderMap
) -> (StatusCode, Json<Value>) {
    let auth_token = headers.get("auth");
    if auth_token.is_none() {
        return (StatusCode::IM_A_TEAPOT, Json(json!({
            "response": "token not present you melon"
        })));
    }
    let auth_token = auth_token.unwrap().to_str().unwrap().to_owned(); 

    let result = get_user(&pool, None, None, Some(auth_token)).await;
    if result.is_err()
    {
        return (StatusCode::BAD_REQUEST, Json(json!({"ERROR" : "bad token"})));
    }
    let user = result.unwrap();
    info!("user: {}\ntested a token", user.username);

    (StatusCode::OK, Json(json!(user.username)))
}