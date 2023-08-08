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

use crate::{utils::{check_token, get_user, check_password_regex, get_friend_status}, structs::{User, FriendRecord, FriendRequest}};


pub fn router() -> Router {
    Router::new()
    //sends or accepts a friend request to or from the player provided 
    .route("/friends/add", get(|| async {"go get some friends"}).post(add_or_accept_friend))
    //removes the friendship of or declines the friend request of from the player provided
    .route("/friends/remove", get(|| async {"are they really *that* bad?"}).post(remove_or_decline_friend))
    
    // .route("/friends/pending", get(|| async {"are they really *that* bad?"}).post(|| async {"TBD"})) //could be added so that you can request to see only the pending requests to accept/deny
    //gets all friends, incoming and outgoing friend requests
    .route("/friends/status/all", get(|| async {"look at how many there are(n't)"}).post(|| async {"TBD"}))

    //gets the status of the friendship or friend request between yourself and another user
    .route("/friends/status", get(friend_status).post(|| async {"TBD"}))
}

// sending friend requests and accepting friend requests can be done through the add route 
// (when adding a friend it checks that you are not already friends with them)

#[derive(Deserialize)]
pub struct FriendStatusRequestParameters {
    username: String,
}




pub async fn friend_status(
    Extension(pool): Extension<PgPool>,
    headers: HeaderMap,
    Json(request): Json<FriendStatusRequestParameters>,
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
    let requesting_user: User = response.unwrap();


    //get the user that is being requested
    let requested_user = get_user(&pool, None, Some(request.username), None).await;
    if requested_user.is_err() {
        return (StatusCode::BAD_REQUEST, Json(json!({
            "response": "user does not exist"
        })))
    }
    let requested_user: User = requested_user.unwrap();

    // log the user making the request
    info!("user: {}, requested to access {}'s status", requesting_user.username, requested_user.username);

    let response_friend_status = get_friend_status(
        &pool,
        requesting_user.id, 
        requested_user.id
        ).await;
    
    if response_friend_status.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
            "response": "friend status could not be gathered"
        })));
    }

    let friend_status = response_friend_status.unwrap();
    
    if friend_status.0.is_some() {
        // if friend Record is returned

        let friendship: FriendRecord = friend_status.0.unwrap();
        info!("{:?}", friendship.acceptance_timestamp.to_string());
        return (StatusCode::OK, Json(json!({
            "response": "success",
            "relation": "friends",
            "friends_since": friendship.acceptance_timestamp.to_string()
        })));

    } else if friend_status.1.is_some() {
        // if friend request is returned

    } else {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
            "response": "an error in fetching a friend request or friendship occured, this may be as 
            there is no relationship between you and the user requested"
        })));
    }



    // TODO ------------------------



    (StatusCode::OK, Json(json!({
        "response": "success",
    })))
}

pub async fn add_or_accept_friend(
    Extension(pool): Extension<PgPool>,
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

    

    return (StatusCode::NOT_IMPLEMENTED, Json(json!({
        "response": "this part of the API is incomplete"
    })))
}

pub async fn remove_or_decline_friend(
    Extension(pool): Extension<PgPool>,
    headers: HeaderMap
) -> (StatusCode, Json<Value>) {



    
    return (StatusCode::NOT_IMPLEMENTED, Json(json!({
        "response": "this part of the API is incomplete"
    })))
}


