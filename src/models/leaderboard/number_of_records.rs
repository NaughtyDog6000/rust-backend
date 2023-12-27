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

/// gets the total number of public records on the leaderboard 
pub async fn total_records(
    Extension(pool): Extension<PgPool>,
) -> (StatusCode, Json<Value>) {
    return handle_error(CustomErrors::Unimplemented);
}