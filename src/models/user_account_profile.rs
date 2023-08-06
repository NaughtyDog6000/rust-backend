use std::string;

use axum::{
    Extension, Json, Router,
    routing::{get, post},
    http::{StatusCode, HeaderMap},
    response::{IntoResponse, Response},
    extract::Query
};

use log::{warn, info, trace, error};
use serde::Deserialize;
use sqlx::{pool, PgPool};
use serde_json::{json, Value};

use crate::{utils::get_user, structs::User};


pub fn router() -> Router {
    Router::new().route("/profile", get(|| async {"this is not done"}).post(|| async {"This does NOT support POST requests"}))
}

//if no query string params are provided, it returns the user making the request's profile

//accepts an optional token and checks the friend status of the requester to the user
//if the profile is set to private return info stating so
// 
pub async fn get_profile(
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


    //validate token
    let response: Result<User, String> =  get_user(&pool, None, None, Some(auth_token)).await;
    if response.is_err() {
        return (StatusCode::BAD_REQUEST, Json(json!({
            "response": "token error"
        })))
    }
    //validate friendsship status

    //validate profile vis

    //get recent games

    //get achievements

    //get stats





    return (StatusCode::OK, Json(json!({
    "response" : "NOT IMPLEMENTED"
    })))
}




pub async fn get_games(

) -> Result<Json<Value>, String> {

    return Ok(Json(json!("none")));
}

pub async fn get_achievements(

) -> Result<Json<Value>, String> {

    return Ok(Json(json!("none")));
}

pub async fn get_player_stats(

) -> Result<Json<Value>, String> {

    return Ok(Json(json!("none")));
}