//This route is for disabling all current tokens associated with the account
// (therefore signing out all current devices)

use std::string;

use axum::{
    extract::Query,
    http::{response, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
};

use log::{error, info, trace, warn};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{pool, PgPool};

use crate::{
    errors::handle_error,
    structs::{Token, User},
    utils::{check_token, check_user_exists, get_user},
};

#[derive(Serialize, Deserialize)]
pub struct DeleteAccountParams {
    pub username: String,
}
// username is required to ensure that someone doesnt accidentally delete the acc
// also means someone who finds a token cant just immediately delete the acc

pub async fn delete_account(
    Extension(pool): Extension<PgPool>,
    headers: HeaderMap,
    Json(request): Json<DeleteAccountParams>,
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

    let user: User;
    let user_req_response = get_user(&pool, None, None, Some(auth_token)).await;

    match user_req_response {
        Ok(usr) => {
            user = usr;
        }
        Err(error) => {
            return handle_error(error);
        }
    }

    if request.username != user.username {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"response": "username does not match user associated with token"})),
        );
    }

    info!("DELETING USER: {}", user.username);
    // -- delete scores --
    let response_scores = sqlx::query(
        "
    DELETE FROM scores
    WHERE user_id = $1
    ",
    )
    .bind(user.id)
    .execute(&pool)
    .await;
    // -- delete tokens --
    let response_tokens = sqlx::query(
        "
    DELETE FROM tokens
    WHERE user_id = $1
    ",
    )
    .bind(user.id)
    .execute(&pool)
    .await;
    // -- delete user --
    let response_users = sqlx::query(
        "
    DELETE FROM users
    WHERE id = $1
    ",
    )
    .bind(user.id)
    .execute(&pool)
    .await;

    println!(
        "scores: {:?}\ntokens: {:?}\nusers: {:?}",
        response_scores, response_tokens, response_users
    );
    // -- check that user no longer exists --

    let user_exists: bool = check_user_exists(&user.username, &pool).await;
    if user_exists {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"response": "an error occured which resulted in the user still existing"})),
        );
    }

    (
        StatusCode::OK,
        Json(json!({"response" : "account deleted successfully"})),
    )
}
