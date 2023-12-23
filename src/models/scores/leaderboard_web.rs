use std::{fmt::Debug, collections::HashMap};
use serde::{Deserialize, Serialize};
use axum::{
    Extension, Json, Router,
    routing::{get, post},
    http::{StatusCode, HeaderMap},
    response::{IntoResponse, Response},
    extract::Query
};

use sqlx::{pool, PgPool, postgres::PgRow};
use serde_json::{json, Value};

use crate::{structs::{VisibilityEnum, GamemodeEnum, OrderByEnum, User}, utils::check_token, errors::{handle_error, CustomErrors}};

const SIGNED_IN_USER_RECORD_MAX: i32 = 50;
const ANONYMOUS_USER_RECORD_MAX: i32 = 10; 



// visibility: global/friends/own
// game_mode: default/Hardcore
// order_by: score, most_recent 
// order: decending/ascending


#[derive(Debug, Deserialize)]
pub struct LeaderboardQueryStringParams {
    visibility: VisibilityEnum,
    game_mode: GamemodeEnum,
    page_length: i32,
    page_offset: i32,
    order_by: OrderByEnum,
    order_ascending: bool,
}

impl Default for LeaderboardQueryStringParams {
    fn default() -> Self {
        Self { 
            page_length: 10, 
            page_offset: 0, 
            visibility: VisibilityEnum::Public, 
            game_mode: GamemodeEnum::Default, 
            order_by: OrderByEnum::Score, 
            order_ascending: false  
        }
    }
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
struct LeaderboardRecord {
    pub username: String,
    pub epoch_upload_time: i64,
    pub epoch_game_start_time: i64,
    pub epoch_game_end_time: i64,
    pub score: i32,
    pub game_mode: String,
}
pub fn router() -> Router {
    Router::new().route("/scores",
        get(leaderboard_web)
        .post(|| async {"This does NOT support POST requests"})
    )
}

///
/// 
pub async fn leaderboard_web(
    Extension(pool): Extension<PgPool>,
    headers: HeaderMap,
    query_params: Option<Query<LeaderboardQueryStringParams>>,
) -> (StatusCode, Json<Value>) {

    let Query(query_params) = query_params.unwrap_or_default();

    let auth_header = headers.get("auth");
    if auth_header.is_none() {
        // user is not signed in
        // only allow public leaderboard access
        // only allow  10 records to be requested at a time

        if query_params.page_length > ANONYMOUS_USER_RECORD_MAX {
            return (StatusCode::BAD_REQUEST, Json(json!({
                "response": "anonymous/non-signed in users can only request 10 records at a time"
            })));
        }
        
        // if a person who is not signed in is trying to see their own/friends records...
        if query_params.visibility != VisibilityEnum::Public {
            return (StatusCode::BAD_REQUEST, Json(json!({
                "response": "non-signed in users cannot see their own/friends records becuase they have none"
            })))
        }
    } else {
        let auth_token = auth_header.unwrap().to_str().unwrap().to_owned();
        if !check_token(&pool, auth_token).await {
            return handle_error(CustomErrors::BadToken);
        }
        
        if query_params.page_length > SIGNED_IN_USER_RECORD_MAX {
            return handle_error(CustomErrors::RequestAmount);
        }
    }

    // CURRENT IMPL DOES NOT SUPPORT VIEWING FRIENDS SCORES & GAMEMODES OTHER THAN DEFAULT
    if query_params.visibility != VisibilityEnum::Public || query_params.game_mode != GamemodeEnum::Default {
        return handle_error(CustomErrors::Unimplemented);
    } 

    // if change the order by depending on if the request is for ascending or decending order
    let mut order: String = String::from("DESC");
    if query_params.order_ascending {
        order = String::from("ASC");
    }

    let res = sqlx::query_as::<_, LeaderboardRecord>("
    SELECT 
        scores.epoch_upload_time,
        scores.epoch_game_start_time,
        scores.epoch_game_end_time,
        scores.score,
        scores.game_mode,
        users.username
    FROM scores
    JOIN users ON scores.user_id = users.id
    ORDER BY scores.score DESC
    LIMIT 10 OFFSET 0;

     ")
    .fetch_all(&pool)
    .await;
    if res.is_err() { return handle_error(CustomErrors::SQLXError(res.unwrap_err())); }
    let response = res.unwrap();


    println!("{:#?}", response);

    return (StatusCode::OK, Json(json!(response)));
}
