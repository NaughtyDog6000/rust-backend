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

use crate::structs::Score;

#[derive(Deserialize)]
pub struct LeaderboardQueryStringParams {
    length: usize,
    offset: usize,
}


// as u can guess it is the default value should it be undefined
impl Default for LeaderboardQueryStringParams {
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
    query_params: Option<Query<LeaderboardQueryStringParams>>,
) -> (StatusCode, Json<Value>) {

    // -- GET THE QUERY PARAMETERS --
    //all paramaters must be provided otherwise it defaults
    let Query(query_params) = query_params.unwrap_or_default();
    let length: i32 = query_params.length as i32;
    let offset: i32 = query_params.offset as i32;

    // -- check for TOKEN to allow the request/higher request amounts --
    warn!("chacking for token not currently being done");

    // -- apply limitations (min & max request amounts) -- 

    if length == 0 {
        return (StatusCode::BAD_REQUEST, Json(json!("length (number of records requested) cannot be 0")));
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

    let response = res.unwrap(); 
    info!("response: {:?}", &response);
    // (StatusCode::OK, Json(json!(format!("length: {}, offset: {}.",length,offset))))
    (StatusCode::OK, Json(json!(response)))

}