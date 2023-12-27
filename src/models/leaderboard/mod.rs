use axum::{Router, routing::{get, post}};
use crate::errors::{handle_error, CustomErrors};

use self::{number_of_records::total_records, user_position::user_position, upload_score::upload_score};
use self::leaderboard::leaderboard;

pub mod leaderboard;
pub mod upload_score;
pub mod user_position;
pub mod number_of_records;

pub fn router() -> Router {
    Router::new()
    .route("/leaderboard/scores",get(upload_score))
    .route("/leaderboard/number_of_records",get(total_records))
    .route("/leaderboard/position", get(user_position))
    .route("/leaderboard/upload", post(leaderboard))
}
