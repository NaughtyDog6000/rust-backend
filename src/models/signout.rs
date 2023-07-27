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
use serde::{Deserialize, __private::size_hint::from_bounds};
use sqlx::{pool, PgPool};
use serde_json::{json, Value};

use crate::{structs::{get_timestamp, TokenRequestParams}, utils::{get_user, check_token}};

use super::leaderboard::LeaderboardQueryStringParams;




pub fn router() -> Router {
    Router::new()
    .route("/signout_all", get(|| async {"this is not done"}).post(signout_all))
    .route("/signout", get(|| async {"This route is incomplete"}).post(signout))
        
    

    
}

// signs out all tokens for the associated accounts
pub async fn signout_all(
    Extension(pool): Extension<PgPool>,
    Json(request): Json<TokenRequestParams>
) -> (StatusCode, Json<Value>) {
    let user = get_user(&pool, None, None, Some(request.token)).await;
    if user.is_err() {
        warn!("invalid token used");
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!("an error occured in finding the user associated with the token")));
    }
        

    let user = user.unwrap();
    let current_time = get_timestamp();
    sqlx::query("DELETE FROM tokens
                WHERE user_id = $1")
                .bind(user.id)
                .execute(&pool)
                .await;


    (StatusCode::OK, Json(json!("signed out/invalidated all current tokens associated with your account")))
}

// signs out the current token
pub async fn signout(
    Extension(pool): Extension<PgPool>,
    Json(request): Json<TokenRequestParams>
) -> (StatusCode, Json<Value>) {
    let valid_token: bool = check_token(&pool, request.token.clone()).await; // this isnt necessary?
    if !valid_token 
    {
        warn!("signout attempt of a token that is either invalid or doesnt exist");
        return (StatusCode::BAD_REQUEST, 
            Json(json!(String::from("this token doesnt exist or is already invalid")))
        )

    }


    let response = sqlx::query("DELETE FROM tokens
    WHERE token = $1")
    .bind(request.token)
    .execute(&pool).await;

    if response.is_err() {
        error!("an unexpected error occured in the deletion of a token");
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!("umm idk what heppend but its not gud")));
    }
    let rows_affected = response.unwrap().rows_affected();
    trace!("rows effected: {}", &rows_affected);

    let success_string: String = format!("ALL {} tokens successfully disabled", rows_affected);

    return (StatusCode::OK, Json(json!(success_string)));
}