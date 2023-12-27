use axum::{Router, routing::{get, post}};
use self::{number_of_records::total_records, user_position::user_position, upload_score::upload_score};
use self::leaderboard::leaderboard;

pub mod leaderboard;
pub mod upload_score;
pub mod user_position;
pub mod number_of_records;

pub fn router() -> Router {
    Router::new()
    .route("/leaderboard/scores",get(upload_score).post(|| async {"This does NOT support POST requests"}))
    .route("/leaderboard/number_of_records",get(total_records).post(|| async {"This is a READ ONLY Route for the total number of public records in the database"}))
    .route("/leaderboard/position", get(user_position).post(|| async {"This does NOT support POST requests"}))
    .route("/leaderboard/upload", get(|| async {"this [POST] route is for uploading your game stats to the leaderboard"}).post(leaderboard))

}