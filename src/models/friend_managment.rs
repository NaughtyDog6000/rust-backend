// this route is for adding, removing 
use std::string;

use axum::{
    Extension, Json, Router,
    routing::{get, post},
    http::{StatusCode, HeaderMap, status, request},
    response::{IntoResponse, Response},
    extract::{Query, path, Path}
};

use chrono::NaiveDate;
use log::{warn, info, trace, error};
use serde::Deserialize;
use sqlx::{pool, PgPool};
use serde_json::{json, Value};

use crate::{utils::{check_token, get_user, check_password_regex, get_friend_status, add_or_accept_friend}, structs::{User, FriendRecord, FriendRequest, FriendStatusEnum}};

/// ## contains the routes for getting and managing relationships between users<br>
/// "friends/add" - adds or sends a friend request to the user passed<br>
/// "friends/remove" - removes or declines the friend request of the user passed<br>
/// "friends/status" - gets the relationship status of the friend passed (added timestamp & friends/requested/none)<br>
/// "friends/status/all"- gets the status of all users currently related to the user requesting (friends, requested)
pub fn router() -> Router {
    Router::new()
    //sends or accepts a friend request to or from the player provided 
    .route("/friends/add", get(|| async {"go get some friends"}).post(add_or_accept_friend_route))
    //removes the friendship of or declines the friend request of from the player provided
    .route("/friends/remove", get(|| async {"are they really *that* bad?"}).post(remove_or_decline_friend_route))
    
    // .route("/friends/pending", get(|| async {"are they really *that* bad?"}).post(|| async {"TBD"})) //could be added so that you can request to see only the pending requests to accept/deny
    //gets all friends, incoming and outgoing friend requests
    .route("/friends/status/all", get(|| async {"look at how many there are(n't)"}).post(|| async {"TBD"}))

    //gets the status of the friendship or friend request between yourself and another user
    .route("/friends/status", get(friend_status).post(|| async {"why even check, we all know you aint got no friends"}))
}

// sending friend requests and accepting friend requests can be done through the add route 
// (when adding a friend it checks that you are not already friends with them)

#[derive(Deserialize)]
pub struct UserQueryParameters {
    username: String,
}

/// gets the 
pub async fn friend_status(
    Extension(pool): Extension<PgPool>,
    headers: HeaderMap,
    query_string: Option<Query<UserQueryParameters>>,
    request: Option<Json<UserQueryParameters>>,
) -> (StatusCode, Json<Value>) {
    let requested_username: String;

    if request.is_some() {
    // -- try to get the user from the body 
        // println!("body used");
        requested_username = request.unwrap().username.to_string();
    } else if query_string.is_some() {
    // -- try to get user from the query string --
        // println!("query string used");
        requested_username = query_string.unwrap().0.username;
    } else {
        // println!("no body or query string provided");
        return (StatusCode::BAD_REQUEST, Json(json!({
            "response": "no body or query string with username parameter provided"
        })));
    }
    
    
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
    let requested_user = get_user(&pool, None, Some(requested_username), None).await;
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
    
    if friend_status.record.is_some() {
        // if friend Record is returned

        let friendship: FriendRecord = friend_status.record.unwrap();
        info!("{:?}", friendship.acceptance_timestamp.to_string());
        return (StatusCode::OK, Json(json!({
            "response": "success",
            "relation": "friends",
            "details": friendship,

        })));

    } else if friend_status.request.is_some() {
        // if friend request is returned
        let friend_request: FriendRequest = friend_status.request.unwrap();

        return (StatusCode::OK, Json(json!({
            "response": "friend request between the two users found",
            "relation": "requested",
            "details": friend_request

        })))

    } else {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
            "response": "an error in fetching a friend request or friendship occured, this may be as 
            there is no relationship between you and the user requested",
            "relation": null,
            "details": null,
        })));
    }

    (StatusCode::OK, Json(json!({
        "response": "success",
    })))
}

/// route used to send or accept a friend request<br>
/// accepts the username passed through either:<br>
/// the query string: .../add?username="nd6k"<br>
/// or the body: {"username": "nd6k"}
pub async fn add_or_accept_friend_route(
    Extension(pool): Extension<PgPool>,
    headers: HeaderMap,
    query_string: Option<Query<UserQueryParameters>>,
    request: Option<Json<UserQueryParameters>>,
) -> (StatusCode, Json<Value>) {
    // -- get token from headers -- 
    let auth_token = headers.get("auth");
    if auth_token.is_none() {
        return (StatusCode::IM_A_TEAPOT, Json(json!({
            "response": "token not present you melon"
        })));
    }
    let auth_token: String = auth_token.unwrap().to_str().unwrap().to_owned(); 

    // -- get the username of the person being added from body or query string --
    let requested_username: String;

    if request.is_some() {
    // -- try to get the user from the body 
        // println!("body used");
        requested_username = request.unwrap().username.to_string();
    } else if query_string.is_some() {
    // -- try to get user from the query string --
        // println!("query string used");
        requested_username = query_string.unwrap().0.username;
    } else {
    // -- no username is provided so cannot proceed --
        // println!("no body or query string provided");
        return (StatusCode::BAD_REQUEST, Json(json!({
            "response": "no body or query string with username parameter provided"
        })));
    }

    // -- get the user who made the request --
    let requesting_user_resp = get_user(&pool, None, None, Some(auth_token)).await; 
    if requesting_user_resp.is_err() {
        return (StatusCode::BAD_REQUEST, Json(json!({
            "response": "bad token"
        })))
    }

    let requesting_user: User = requesting_user_resp.unwrap();



    
    // -- get the user who is being requested to be added or friend requested
    let requested_user_response = get_user(&pool, None, Some(requested_username), None).await;
    if requested_user_response.is_err() {
        return (StatusCode::BAD_REQUEST, Json(json!({
            "response": "the user being requested does not exist"
        })));
    }
    let requested_user: User = requested_user_response.unwrap();

    let status: Result<FriendStatusEnum, String> = add_or_accept_friend(&pool, requesting_user.id, requested_user.id).await;
    if status.is_err() {

        let status: String = status.unwrap_err();
        return (StatusCode::OK, Json(json!({
            "response": "error",
            "details": status,
        })));
    }
    let status = status.unwrap();


    return (StatusCode::ACCEPTED, Json(json!({
        "response": "success",
        "status": status
    })))
}

pub async fn remove_or_decline_friend_route(
    Extension(pool): Extension<PgPool>,
    headers: HeaderMap,
    Json(request): Json<UserQueryParameters>,
) -> (StatusCode, Json<Value>) {


    todo!();

    
    return (StatusCode::NOT_IMPLEMENTED, Json(json!({
        "response": "this part of the API is incomplete"
    })))
}


