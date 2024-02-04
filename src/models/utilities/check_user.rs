use axum::{
    Extension, Json, Router,
    routing::{get, post},
    http::{StatusCode, HeaderMap},
    response::{IntoResponse, Response},
    extract::Query
};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::{errors::{handle_error, CustomErrors}, utils::get_user};

#[derive(Deserialize)]
pub struct CheckUserParameters {
    username: String
}
// TODO!() in future return the user type and some extra info such as the relationship between the user requesting
//  and the requestee

// checks if the user exists 
pub async fn check_user(
    Extension(pool): Extension<PgPool>,
    headers: HeaderMap,
    request: Option<Json<CheckUserParameters>>,

) -> (StatusCode, Json<Value>) {
    // -- get token from headers -- 
    let auth_token = headers.get("auth");
    if auth_token.is_none() {
        return (StatusCode::IM_A_TEAPOT, Json(json!({
            "response": "token not present you melon"
        })));
    }
    let auth_token = auth_token.unwrap().to_str().unwrap().to_owned(); 

    if (request.is_none())
    {
        return handle_error(CustomErrors::MissingQueryParams { missing_params: String::from("username") });
    }
    let requested_username = request.unwrap().username.to_string();

    let user = get_user(&pool, None, Some(requested_username), None).await;
    match user {
        Ok(_) => return (StatusCode::OK, Json(json!({
            "data": {
                "isUser": true,
            }
        }))),
        Err(_) => return (StatusCode::OK, Json(json!({"data": {
            "isUser": false,
        }}))),
    }
}