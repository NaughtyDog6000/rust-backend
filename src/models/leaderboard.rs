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
    query_params: Option<Query<QueryParams>>,
) -> (StatusCode, Json<Value>) {
    //both paramaters must be provided otherwise it defaults
    let Query(query_params) = query_params.unwrap_or_default();
    let length: i32 = query_params.length as i32;
    if length == 0 {
        return (StatusCode::BAD_REQUEST, Json(json!("length (number of records requested) cannot be 0")));
    }

    let offset: i32 = query_params.offset as i32;

    let res: Result<Vec<Score>, sqlx::Error>;

    res = sqlx::query_as::<_, Score>(
        "SELECT * FROM scores
        LIMIT $1 OFFSET $2"
    )
    .bind(&length)
    .bind(&offset)
    .fetch_all(&pool).await;

    // println!("response: {:?}", &res.unwrap());

    // (StatusCode::OK, Json(json!(format!("length: {}, offset: {}.",length,offset))))
    (StatusCode::OK, Json(json!(format!("{:?}",res.unwrap()))))

}