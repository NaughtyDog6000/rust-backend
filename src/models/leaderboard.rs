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


// as u can guess it is the default value should it be undefined
impl Default for QueryParams {
    fn default() -> Self {
        Self { length: 10, offset: 0}
    }
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
    let length = query_params.length;
    let offset = query_params.offset;

    (StatusCode::OK, Json(json!(format!("length: {}, offset: {}.",length,offset))))
}