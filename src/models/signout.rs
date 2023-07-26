//This route is for disabling all current tokens associated with the account 
// (therefore signing out all current devices)

use std::string;

use axum::{
    Extension, Json, Router,
    routing::{get, post},
    http::StatusCode,
    response::{IntoResponse, Response},
    extract::Query
};

use log::{warn, info, trace, error};
use serde::Deserialize;
use sqlx::{pool, PgPool};
use serde_json::{json, Value};

use crate::{structs::get_timestamp, utils::{get_user, check_token}};

use super::{leaderboard::LeaderboardQueryStringParams, test_token::TokenRequestParams};




pub fn router() -> Router {
    Router::new()
    .route("/signout_all", get(|| async {"this is not done"}).post(signout_all))
    .route("/signout", get(|| async {"This route is incomplete"}).post(signout))
        
    

    
}

pub async fn signout_all(
    Extension(pool): Extension<PgPool>,
    Json(request): Json<TokenRequestParams>
) -> (StatusCode, Json<Value>) {
    let user = check_token(&pool, request.token).await;
    match user {
       Ok(_) => {}
       Err(_) => { return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!("an error occured in finding the user associated with the token")));}
    }

    let user = user.unwrap();
    let current_time = get_timestamp();
    sqlx::query("UPDATE users
                SET epoch_invalidate_tokens = $1
                WHERE id = $2")
                .bind(current_time)
                .bind(user.id)
                .execute(&pool);


    (StatusCode::OK, Json(json!("signed out/invalidated all current tokens associated with your account")))
}

pub async fn signout(
    Extension(pool): Extension<PgPool>,
    Json(request): Json<TokenRequestParams>
) -> (StatusCode, Json<Value>) {


    warn!("singout not currently working");
    return (StatusCode::OK, Json(json!("signout successful")));
}