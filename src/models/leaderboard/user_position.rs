use axum::{
    extract::Query,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{pool, postgres::PgRow, PgPool};

use crate::errors::{handle_error, CustomErrors};

/// gets the total number of public records on the leaderboard
pub async fn user_position(Extension(pool): Extension<PgPool>) -> (StatusCode, Json<Value>) {
    return handle_error(CustomErrors::Unimplemented);
}
