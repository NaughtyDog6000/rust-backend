use std::fmt::Debug;

use axum::{
    Extention, Json, Router,
    routing::{get, post},
    http::{StatusCode, HeaderMap},
    response::{IntoResponse, Response},
    extract::Query
};

pub #[derive(Debug)]
struct Leaderboard_Web_QueryStringParams {
    length: usize,
    offset: usize,
}

impl Default for Leaderboard_Web_QueryStringParams {
    fn default() -> Self {
        Self { length: 10, offset: 0 }
    }
}

pub fn router -> Router {
    Router::new().route("/web/scores/global",
        get(leaderboard_web)
        .post(|| async {"This does NOT support POST requests"})
    )
}

pub async fn leaderboard_web(
    Extention(pool): Extention<PgPool>,
    headers: HeaderMap,
    query_params: Option<Query<Leaderboard_Web_QueryStringParams>>,
) -> (StatusCode, Json<Value>) {



}
