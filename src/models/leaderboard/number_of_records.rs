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

use crate::errors::{handle_error, CustomErrors};

use super::leaderboard::{TotalRecords, LeaderboardQueryParams};

/// gets the total number of public records on the leaderboard
/// 
/// Does not require Auth 
pub async fn total_records(
    Extension(pool): Extension<PgPool>,
) -> (StatusCode, Json<Value>) {
    let response = sqlx::query_as::<_, TotalRecords>("
        SELECT COUNT(*) AS total_records
        FROM scores
        JOIN users ON scores.user_id = users.id;
    ").fetch_one(&pool).await;

    let total_records = response.unwrap();

    return (StatusCode::OK, Json(json!(
        total_records
    )));

}


/// gets the total number of records on the leaderboard meeting the filter conditions passed
pub async fn total_records_custom(
    Extension(pool): Extension<PgPool>,
    headers: HeaderMap,
    Json(body_params): Json<Option<LeaderboardQueryParams>>
) -> (StatusCode, Json<Value>) {
    let body_params: LeaderboardQueryParams = body_params.unwrap_or_default();

    let auth_header = headers.get("auth");

    if auth_header.is_none() {
        return handle_error(CustomErrors::RequiresAuthorisation)
    }

    let response = sqlx::query_as::<_, TotalRecords>("
    SELECT COUNT(*) AS total_records
    FROM scores
    JOIN users ON scores.user_id = users.id;
    WHERE 
        scores.game_mode = $1 AND
        scores.visibility = $2 AND
        scores.epoch_upload_time BETWEEN $3 AND $4
        
    ")
    .bind(&body_params.game_mode.to_string())
    .bind(&body_params.visibility.to_string())
    .bind(&body_params.uploaded_after)
    .bind(&body_params.uploaded_before)
    .fetch_one(&pool)
    .await;

    if response.is_err() {
        return handle_error(CustomErrors::SQLXError(response.unwrap_err()));
    }
    let count = response.unwrap();

    return (StatusCode::OK, Json(json!(count)));
}

/// returns the number of public records in the database
/// Optionally 
pub fn get_total_number_of_records(
    pool: &PgPool,
    user_id: Option<i64>
) {
    if user_id.is_some() {

    }
}