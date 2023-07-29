// this route is for adding, removing 
use std::string;

use axum::{
    Extension, Json, Router,
    routing::{get, post},
    http::StatusCode,
    response::{IntoResponse, Response},
    extract::Query
};

use chrono::NaiveDate;
use log::{warn, info, trace, error};
use serde::Deserialize;
use sqlx::{pool, PgPool};
use serde_json::{json, Value};

use crate::{utils::{check_token, get_user, check_password_regex}, structs::User};


pub fn router() -> Router {
    Router::new()
    .route("/account/friends/add", get(|| async {"go get some friends"}).post(|| async {"TBD"}))
    .route("/account/friends/remove", get(|| async {"are they really *that* bad?"}).post(|| async {"TBD"}))
}

// sending friend requests and accepting friend requests can be done through the add route 
// (when adding a friend it checks that you are not already friends with them)