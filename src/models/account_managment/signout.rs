//This route is for disabling all current tokens associated with the account
// (therefore signing out all current devices)

use std::string;

use axum::{
    extract::Query,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
};

use log::{error, info, trace, warn};
use log4rs::encode::json;
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::{pool, PgPool};

use crate::utils::{check_token, get_timestamp, get_user};

pub fn router() -> Router {
    Router::new()
}

// signs out all tokens for the associated accounts
pub async fn signout_all(
    Extension(pool): Extension<PgPool>,
    headers: HeaderMap,
) -> (StatusCode, Json<Value>) {
    // -- get token from headers --
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

    let user = get_user(&pool, None, None, Some(auth_token)).await;
    if user.is_err() {
        warn!("invalid token used");
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
            "response": "this token doesn't exist or is already invalid"
            })),
        );
        // return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!("an error occured in finding the user associated with the token")));
    }

    let user = user.unwrap();
    let current_time = get_timestamp();
    let response = sqlx::query(
        "DELETE FROM tokens
                WHERE user_id = $1",
    )
    .bind(user.id)
    .execute(&pool)
    .await;

    let rows_affected = response.unwrap().rows_affected();

    (
        StatusCode::OK,
        Json(json!({
            "signout": "success",
            "number": rows_affected
        })),
    )
}

// signs out the current token
pub async fn signout(
    Extension(pool): Extension<PgPool>,
    headers: HeaderMap,
) -> (StatusCode, Json<Value>) {
    // -- get token from headers --
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

    let valid_token: bool = check_token(&pool, auth_token.clone()).await; // this isnt necessary?
    if !valid_token {
        warn!("signout attempt of a token that is either invalid or doesnt exist");
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
            "response": "this token doesn't exist or is already invalid"
            })),
        );
    }

    let response = sqlx::query(
        "DELETE FROM tokens
    WHERE token = $1",
    )
    .bind(auth_token)
    .execute(&pool)
    .await;

    if response.is_err() {
        error!("an unexpected error occured in the deletion of a token");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!("umm idk what heppend but its not gud")),
        );
    }
    let rows_affected = response.unwrap().rows_affected();
    trace!("rows effected: {}", &rows_affected);

    (
        StatusCode::OK,
        Json(json!({
            "response": "success",
            "number": rows_affected
        })),
    )
}
