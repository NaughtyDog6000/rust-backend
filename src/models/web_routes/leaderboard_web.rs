use std::fmt::Debug;

use axum::{
    Extension, Json, Router,
    routing::{get, post},
    http::{StatusCode, HeaderMap},
    response::{IntoResponse, Response},
    extract::Query
};

use sqlx::{pool, PgPool};
use serde_json::{json, Value};


#[derive(Debug)]
pub struct LeaderboardWebQueryStringParams {
    length: usize,
    offset: usize,
}

impl Default for LeaderboardWebQueryStringParams {
    fn default() -> Self {
        Self { length: 10, offset: 0 }
    }
}

pub fn router() -> Router {
    Router::new().route("/web/scores/global",
        get(leaderboard_web)
        .post(|| async {"This does NOT support POST requests"})
    )
}

pub async fn leaderboard_web(
    Extension(pool): Extension<PgPool>,
    headers: HeaderMap,
    query_params: Option<Query<LeaderboardWebQueryStringParams>>,
) -> (StatusCode, Json<Value>) {


    return (StatusCode::OK, Json(json!({"response": "trest"})));
}
