//This route is for disabling all current tokens associated with the account 
// (therefore signing out all current devices)

use std::string;

use axum::{
    Extension, Json, Router,
    routing::{get, post},
    http::{StatusCode, response},
    response::{IntoResponse, Response},
    extract::Query
};

use log::{warn, info, trace, error};
use serde::{Deserialize, Serialize};
use sqlx::{pool, PgPool};
use serde_json::{json, Value};

use crate::{structs::{User, Token}, utils::{check_token, get_user}};

use super::signup::check_user_exists;


pub fn router() -> Router {
    Router::new().route("/profile/delete_me",
        get(delete_account)
        .post(delete_account)
    )
}

#[derive(Serialize, Deserialize)]
pub struct DeleteAccountParams {
    pub token: String,
    pub username: String 
} 
// username is required to ensure that someone doesnt accidentally delete the acc
// also means someone who finds a token cant just immediately delete the acc

pub async fn delete_account(
    Extension(pool): Extension<PgPool>,
    Json(request): Json<DeleteAccountParams>
) -> (StatusCode, Json<Value>) {
    let user_req = get_user(&pool, None, None, Some(request.token)).await;

    if user_req.is_err() {
        return (StatusCode::BAD_REQUEST, Json(json!({"response": user_req.unwrap_err()})));
    }
    let user: User = user_req.unwrap();

    if request.username != user.username {
        return (StatusCode::BAD_REQUEST, Json(json!({"response": "username does not match user associated with token"})));
    }

    info!("DELETING USER: {}", user.username);
    // -- delete scores --
    sqlx::query("
    DELETE FROM scores
    WHERE user_id = $1
    ")
    .bind(user.id)
    .execute(&pool)
    .await;
    // -- delete tokens --
    sqlx::query("
    DELETE FROM tokens
    WHERE user_id = $1
    ")
    .bind(user.id)
    .execute(&pool)
    .await;
    // -- delete user --
    sqlx::query("
    DELETE FROM users
    WHERE id = $1
    ")
    .bind(user.id)
    .execute(&pool)
    .await;

    // -- check that user no longer exists --

    let user_exists: bool = check_user_exists(&user.username, &pool).await;
    if user_exists {return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"response": "an error occured which resulted in the user still existing"}))) }

    (StatusCode::OK, Json(json!({"response" : "account deleted successfully"})))
}