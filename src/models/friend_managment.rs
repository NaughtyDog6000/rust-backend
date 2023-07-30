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
    .route("/friends/view/all", get(|| async {"look at how many there are(n't)"}).post(|| async {"TBD"}))
    .route("/friends/view", get(friend_status_query_strings).post(|| async {"TBD"}))
}

// sending friend requests and accepting friend requests can be done through the add route 
// (when adding a friend it checks that you are not already friends with them)

#[derive(Deserialize)]
pub struct GetFriendQueryStringParams {
    user: String,
}

pub async fn friend_status_query_strings(
params: Query<GetFriendQueryStringParams>,
headers: HeaderMap
) -> (StatusCode, Json<Value>) {
    let request: GetFriendQueryStringParams = params.0;  // this is wierd https://docs.rs/axum/latest/axum/extract/struct.Query.html

    println!("{:?}",request.user);
    println!("{:?}", headers);

    (StatusCode::OK, Json(json!({
        "response": "success",
    })))
}