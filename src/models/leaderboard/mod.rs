use crate::errors::{handle_error, CustomErrors};
use axum::{
    routing::{get, post},
    Router,
};

use self::leaderboard::leaderboard;
use self::{
    number_of_records::{total_records, total_records_custom},
    upload_score::upload_score,
    user_position::user_position,
};

pub mod leaderboard;
pub mod number_of_records;
pub mod upload_score;
pub mod user_position;

pub fn router() -> Router {
    Router::new()
        .route("/leaderboard/scores", get(leaderboard).post(leaderboard))
        .route("/leaderboard/number_of_records", get(total_records))
        .route(
            "/leaderboard/number_of_records/custom",
            get(total_records_custom),
        )
        .route("/leaderboard/position", get(user_position))
        .route("/leaderboard/upload", post(upload_score))
}
