use std::string;

use axum::{
    Extension, Json, Router,
    routing::{get, post},
    http::{StatusCode, HeaderMap},
    response::{IntoResponse, Response},
    extract::Query
};

use log::{warn, info, trace, error};
use serde::Deserialize;
use sqlx::{pool, PgPool};
use serde_json::{json, Value};

use crate::{structs::{Score, User}, 
            utils::{check_token, get_user, get_timestamp, get_datetime_utc}, errors::handle_error};


#[derive(Deserialize)]
pub struct UploadScoreRequestParams {
    score: i64,
    gamemode: String,
    epoch_game_start_time: i64,
    epoch_game_end_time: i64,
}

pub fn router() -> Router {
    Router::new().route("/scores/upload",
        get(|| async {"this [POST] route is for uploading your game stats to the leaderboard"})
        .post(leaderboard)
    )
}

pub async fn leaderboard(    
    Extension(pool): Extension<PgPool>,
    headers: HeaderMap,
    Json(request): Json<UploadScoreRequestParams>,
) -> (StatusCode, Json<Value>) {
    let auth_token = headers.get("auth");
    if auth_token.is_none() {
        return (StatusCode::BAD_REQUEST, Json(json!({
            "response": "token not present you melon"
        })));
    }
    let auth_token: String = auth_token.unwrap().to_str().unwrap().to_owned(); 

    // -- parse the Request Params into struct --

    info!("SCORE UPLOAD REQUEST PARAMETERS:\nTOKEN: {},\nSCORE: {},\nGamemode: {},\nGameStart: {}\nGameEnd: {}",
    auth_token, request.score, request.gamemode,
    request.epoch_game_start_time, request.epoch_game_end_time);

    // -- check for TOKEN --

    let token_valid: bool = true;

    let user: User;

    // -- extract user_id from token --
    let result = get_user(&pool, None, None, Some(auth_token)).await;
    if result.is_err() {
        return handle_error(result.unwrap_err());
    }
    let user: User = result.unwrap();
    // -- apply limitations (request spam & invalid scores check) -- 
    
    warn!("limitations for spam and score validation checks not complete");

    // -- make query to database for a matching game to prevent double upload --


    let res = sqlx::query_as::<_, Score>(
        "SELECT * FROM scores
        WHERE user_id = $1 AND epoch_game_start_time = $2;" //select one where user id & start time matches
    )
    .bind(user.id)
    .bind(request.epoch_game_start_time)
    .fetch_one(&pool).await;

    // if it already exists return an error (already uploaded).
    if res.is_ok() {
        info!("response: {:?}", &res.unwrap());
        return (StatusCode::ALREADY_REPORTED, 
            Json(json!("this score has already been uploaded")));
    }
    
    // if  { return (StatusCode::ALREADY_REPORTED, Json(json!("this game was already uploaded"))); },


    // upload the record to the scores table


    let resp = sqlx::query("
    INSERT INTO scores (user_id, score, game_mode, epoch_upload_time, epoch_game_start_time, epoch_game_end_time)
    VALUES ($1, $2, $3, $4, $5, $6)
    ")
    .bind(user.id)
    .bind(request.score)
    .bind(request.gamemode)
    .bind(get_timestamp())
    .bind(request.epoch_game_start_time)
    .bind(request.epoch_game_end_time)
    .execute(&pool).await;

    info!("{:?}", resp);
    // (StatusCode::OK, Json(json!(format!("length: {}, offset: {}.",length,offset))))
    (StatusCode::OK, Json(json!("score record creation successful")))

}