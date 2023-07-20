use axum::{
    Extension, Json, Router,
    routing::{get, post},
    http::StatusCode,
    response::{IntoResponse, Response},
    extract::Query
};

use jwt_simple::prelude::HS256Key;
use log::{warn, info, trace, error};
use serde::Deserialize;
use sqlx::{pool, PgPool};
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct QueryParams {
    length: usize,
    offset: usize,
}


pub fn router() -> Router {
    Router::new().route("/scores/global/leaderboard",
        get(leaderboard)
        .post(|| async {"This does NOT support POST requests"})
    )
}

pub async fn leaderboard(    
    Extension(key): Extension<HS256Key>,
    Extension(pool): Extension<PgPool>,
    query_params: Query<QueryParams>,
) -> (StatusCode, Json<Value>) {

    (StatusCode::OK, Json(json!("da")))
}