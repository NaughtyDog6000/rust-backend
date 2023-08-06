// this route is for adding, removing 
use std::string;

use axum::{
    Extension, Json, Router,
    routing::{get, post},
    http::{StatusCode, HeaderMap},
    response::{IntoResponse, Response},
    extract::{Query, path, Path}
};

use chrono::NaiveDate;
use log::{warn, info, trace, error};
use serde::Deserialize;
use sqlx::{pool, PgPool};
use serde_json::{json, Value};

use crate::{utils::{check_token, get_user, check_password_regex}, structs::User};


pub fn router() -> Router {
    Router::new()
    .route("/friends/add", get(|| async {"go get some friends"}).post(|| async {"TBD"}))
    .route("/friends/remove", get(|| async {"are they really *that* bad?"}).post(|| async {"TBD"}))
    // .route("/friends/pending", get(|| async {"are they really *that* bad?"}).post(|| async {"TBD"})) //could be added so that you can request to see only the pending requests to accept/deny
    .route("/friends/status/all", get(|| async {"look at how many there are(n't)"}).post(|| async {"TBD"}))
    .route("/friends/status", get(friend_status_query_strings).post(|| async {"TBD"}))
}

// sending friend requests and accepting friend requests can be done through the add route 
// (when adding a friend it checks that you are not already friends with them)

#[derive(Deserialize)]
pub struct GetFriendQueryStringParams {
    user: String,
}

pub async fn friend_status_query_strings(
    Extension(pool): Extension<PgPool>,
    params: Query<GetFriendQueryStringParams>,
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


    // -- validate token --
    let response: Result<User, String> =  get_user(&pool, None, None, Some(auth_token)).await;
    if response.is_err() {
        return (StatusCode::BAD_REQUEST, Json(json!({
            "response": "token error"
        })))
    }

    // -- get the username from the query string
    let request: GetFriendQueryStringParams = params.0;  // this is wierd https://docs.rs/axum/latest/axum/extract/struct.Query.html


    // log the user making the request
    let requesting_user: User = response.unwrap();
    info!("user: {}, requested to access {}'s status", requesting_user.username, request.user);
    // 
    
    

    println!("{:?}",request.user);

    (StatusCode::OK, Json(json!({
        "response": "success",
    })))
}