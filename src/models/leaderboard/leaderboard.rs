use std::{fmt::Debug, collections::HashMap};
use serde::{Deserialize, Serialize};
use axum::{
    Extension, Json,
    http::{StatusCode, HeaderMap},
    response::{IntoResponse, Response},
    extract::Query
};

use sqlx::{pool, PgPool, postgres::PgRow};
use serde_json::{json, Value};

use crate::{structs::{VisibilityEnum, GamemodeEnum, OrderByEnum, User}, utils::{check_token, self, get_timestamp}, errors::{handle_error, CustomErrors}};

const SIGNED_IN_USER_RECORD_MAX: i32 = 50;
const ANONYMOUS_USER_RECORD_MAX: i32 = 10; 



// visibility: global/friends/own
// game_mode: default/Hardcore
// order_by: score, most_recent 
// order: decending/ascending


#[derive(Debug, Deserialize)]
pub struct LeaderboardQueryParams {
    pub visibility: VisibilityEnum,
    pub uploaded_after: i64, 
    pub uploaded_before: i64, 
    pub game_mode: GamemodeEnum,
    pub order_by: OrderByEnum,
    pub order_ascending: bool,

    pub page_length: i32,
    pub page_offset: i32,
}

impl Default for LeaderboardQueryParams {
    fn default() -> Self {
        Self { 
            page_length: 10, 
            page_offset: 0, 
            visibility: VisibilityEnum::Public, 
            game_mode: GamemodeEnum::Default, 
            order_by: OrderByEnum::Score, 
            order_ascending: false,  
            uploaded_after: 0,
            uploaded_before: get_timestamp(),
        }
    }
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct LeaderboardRecord {
    pub username: String,
    pub epoch_upload_time: i64,
    pub epoch_game_start_time: i64,
    pub epoch_game_end_time: i64,
    pub score: i32,
    pub game_mode: String,
}

/// the total records meeting the current filters
#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct TotalRecords {
    pub total_records: i64,
}


struct LeaderboardResponseStruct {
    pub time_sent: i64,
    pub page_length: i32,
    pub page_offset: i32,
    pub total_records: i32,
    pub records: Vec<LeaderboardRecord>,

}


/// 
pub async fn leaderboard(
    Extension(pool): Extension<PgPool>,
    headers: HeaderMap,
    Json(query_params): Json<Option<LeaderboardQueryParams>>
    
) -> (StatusCode, Json<Value>) {
    let query_params = query_params.unwrap_or_default();


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
    let mut order_direction: String = String::from("DESC");
    if query_params.order_ascending {
        order_direction = String::from("ASC");
    }

    let order_condition = format!("{} {}", query_params.order_by.to_string(), order_direction);
    println!("{}", order_condition);

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
    WHERE 
        scores.game_mode = $1 AND
        $2
        scores.epoch_upload_time BETWEEN $3 AND $4 
    ORDER BY $5
    LIMIT $6 OFFSET $7;

     ")
    .bind(&query_params.game_mode.to_string())
    .bind("")
    .bind(&query_params.uploaded_after)
    .bind(&query_params.uploaded_before)
    .bind(&order_condition)
    .bind(&query_params.page_length)
    .bind(&query_params.page_offset)
    .fetch_all(&pool)
    .await;
    if res.is_err() { return handle_error(CustomErrors::SQLXError(res.unwrap_err())); }
    let records = res.unwrap();


    let res = sqlx::query_as::<_, TotalRecords>("
    SELECT COUNT(*) AS total_records
    FROM scores
    JOIN users ON scores.user_id = users.id;
    ").fetch_one(&pool).await;
    if res.is_err() {return handle_error(CustomErrors::SQLXError(res.unwrap_err())); }

    let total_records = res.unwrap();
    println!("{:#?}, \ntotal records {:#?}", records, total_records);

    return (StatusCode::OK, Json(json!({
        "total_records": total_records.total_records,
        "page_records": records,
    })));
}