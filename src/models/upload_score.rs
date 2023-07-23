use std::string;

use axum::{
    Extension, Json, Router,
    routing::{get, post},
    http::StatusCode,
    response::{IntoResponse, Response},
    extract::Query
};

use jwt_simple::prelude::*;
use log::{warn, info, trace, error};
use serde::Deserialize;
use sqlx::{pool, PgPool};
use serde_json::{json, Value};

use crate::structs::{Score, get_timestamp};
use crate::structs::JTWCustomClaims;

#[derive(Deserialize)]
pub struct UploadScoreRequestParams {
    token: String,
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
    Extension(key): Extension<HS256Key>,
    Extension(pool): Extension<PgPool>,
    Json(request): Json<UploadScoreRequestParams>,
) -> (StatusCode, Json<Value>) {
    // -- parse the Request Params into struct --

    info!("SCORE UPLOAD REQUEST PARAMETERS:\nTOKEN: {},\nSCORE: {},\nGamemode: {},\nGameStart: {}\nGameEnd: {}",
    request.token, request.score, request.gamemode,
    request.epoch_game_start_time, request.epoch_game_end_time);

    // -- check for TOKEN --

    let token_valid: bool = true;

    let username: String;
    let user_id: i64;

    // -- extract user_id from token --
    let claims = key.verify_token::<JTWCustomClaims>(&request.token, Default::default());
    match &claims {
        Ok(claims) => {
            username = claims.custom.username.clone();
            user_id = claims.custom.id.clone();
            info!("Signin of: {}, id: {}", &username, &user_id);

        }
        Err(error) => {
            warn!("bad token use attempted");
            return (StatusCode::BAD_REQUEST, Json(json!("bad token")))
        }
    }
    // -- apply limitations (request spam & invalid scores check) -- 
    
    warn!("limitations for spam and score validation checks not complete");

    // -- make query to database for a matching game to prevent double upload --


    let res = sqlx::query_as::<_, Score>(
        "SELECT * FROM scores
        WHERE user_id = $1 AND epoch_game_start_time = $2;" //select one where user id & start time matches
    )
    .bind(&user_id)
    .bind(request.epoch_game_start_time)
    .fetch_one(&pool).await;

    // if it already exists return an error (already uploaded).
    if res.is_ok() {
        info!("response: {:?}", &res.unwrap());
        return (StatusCode::INTERNAL_SERVER_ERROR, 
            Json(json!("an internal error occured? ocured? occurd? while checking if this record already exists")));
    }
    
    // if  { return (StatusCode::ALREADY_REPORTED, Json(json!("this game was already uploaded"))); },


    // upload the record to the scores table


    let resp = sqlx::query("INSERT INTO scores (user_id, score, game_mode, epoch_upload_time, epoch_game_start_time, epoch_game_end_time)
                VALUES ($1, $2, $3, $4, $5, $6)
    ")
    .bind(user_id)
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