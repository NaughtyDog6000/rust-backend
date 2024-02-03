use axum::{
    Extension, Json, Router,
    routing::{get, post},
    http::{StatusCode, HeaderMap},
    response::{IntoResponse, Response},
    extract::Query
};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::errors::{handle_error, CustomErrors};


pub async fn get_user_exists(
    Extension(pool): Extension<PgPool>,
    query_params: Option<Query<Value>>,
    headers: HeaderMap
) -> (StatusCode, Json<Value>) {
    // -- get token from headers -- 
    let auth_token = headers.get("auth");
    if auth_token.is_none() {
        return (StatusCode::IM_A_TEAPOT, Json(json!({
            "response": "token not present you melon"
        })));
    }
    let auth_token = auth_token.unwrap().to_str().unwrap().to_owned(); 

    return handle_error(CustomErrors::Unimplemented);
}