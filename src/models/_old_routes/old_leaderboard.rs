use std::fmt::Debug;

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

use crate::{structs::Score, utils::check_token};

#[derive(Deserialize)]
pub struct LeaderboardQueryStringParams {
    length: usize,
    offset: usize,
}


// as u can guess it is the default value should it be undefined
impl Default for LeaderboardQueryStringParams {
    fn default() -> Self {
        Self { length: 10, offset: 0 }
    }
}

pub async fn leaderboard_old(    
    Extension(pool): Extension<PgPool>,
    headers: HeaderMap,
    query_params: Option<Query<LeaderboardQueryStringParams>>,
) -> (StatusCode, Json<Value>) {

    // -- GET THE QUERY PARAMETERS --
    //all paramaters must be provided otherwise it defaults
    let Query(query_params) = query_params.unwrap_or_default();
    let length: i32 = query_params.length as i32;
    let offset: i32 = query_params.offset as i32;

    // -- check for TOKEN to allow the request/higher request amounts --
    // -- get token from headers -- 
    let auth_token_header = headers.get("auth");

    let mut signed_in: bool = false; // auth status
    let auth_token: String; //token provided

    if auth_token_header.is_none() {
        // return (StatusCode::IM_A_TEAPOT, Json(json!({
        //     "response": "token not present you melon"
        // })));
        info!("anonymous signin access");
    } else {
        auth_token = auth_token_header.unwrap().to_str().unwrap().to_owned(); //unwraps from &headervalue type to string
        signed_in = check_token(&pool, auth_token).await; //verifies the token, returning true or false for the validity 
        if signed_in == false { //creates a log if token is invalid (someone tried to use a token in the headers that was bad)
            warn!("an invalid token use attempt for accessing the leaderboard");
            return (StatusCode::BAD_REQUEST, Json(json!({
                "response": "token used was invalid, please reauthenticate and try again"
            })));
        }
    }


    // -- apply limitations (min & max request amounts) -- 
    
    //signin dependant limitations
    if signed_in {
        if length > 50 {
            return (StatusCode::BAD_REQUEST, Json(json!({
                "response": "users can request a maximum of 50 records at a time"
                
            })));
        }

    } else {
        if length > 10 {
            return (StatusCode::BAD_REQUEST, Json(json!({
                "response": "anonymous users can only request 10 records at a time"
                
            })));
        }
    }


    //global limitations (limitations that apply regardless of signin status)
    if length <= 0 || offset < 0 {
        return (StatusCode::BAD_REQUEST, Json(json!({
            "response": "the length cannot be 0 or less, and/or the offset cannot be less than zero"
            
        })));
    }

    // -- make query to database for the records (scores) requested --

    let res: Result<Vec<Score>, sqlx::Error>;

    res = sqlx::query_as::<_, Score>(
        "SELECT * FROM scores
        LIMIT $1 OFFSET $2"
    )
    .bind(&length)
    .bind(&offset)
    .fetch_all(&pool).await;

    let response: Vec<Score> = res.unwrap(); 
    info!("response: {:?}", &response);
    // (StatusCode::OK, Json(json!(format!("length: {}, offset: {}.",length,offset))))
    (StatusCode::OK, Json(json!(response)))

}