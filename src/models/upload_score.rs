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


pub fn router() -> Router {
    Router::new().route("/scores/upload",
        get(|| async {""})
        .post(|| async {"This does NOT support POST requests"})
    )
}

pub async fn leaderboard(    
    Extension(key): Extension<HS256Key>,
    Extension(pool): Extension<PgPool>,
) -> (StatusCode, Json<Value>) {

    // -- check for TOKEN to allow the request/higher request amounts --
    warn!("chacking for token not currently being done");

    // -- apply limitations (request spam & invalid scores check) -- 



    // -- make query to database for a matching game to prevent double upload --


    let res = sqlx::query_as::<_, Score>(
        "SELECT * FROM scores
        WHERE epoch_game_start_time = $1 " //select one where user id & start time matches
    )
    .bind(&length)
    .bind(&offset)
    .fetch_optional(&pool).await;

    let response = res.unwrap(); 
    info!("response: {:?}", &response);


    // if it already exists return error.




    // (StatusCode::OK, Json(json!(format!("length: {}, offset: {}.",length,offset))))
    (StatusCode::OK, Json(json!(response)))

}