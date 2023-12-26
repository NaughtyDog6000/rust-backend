use axum::{Router, routing::{get, post}};
use self::{number_of_records::total_records, user_position::user_position};
use self::leaderboard_web::leaderboard_web;

pub mod leaderboard_web;
pub mod number_of_records;
pub mod user_position;

pub fn router() -> Router {
    Router::new()
    .route("/leaderboard/scores",get(leaderboard_web).post(|| async {"This does NOT support POST requests"}))
    .route("/leaderboard/number_of_records",get(total_records).post(|| async {"This is a READ ONLY Route for the total number of public records in the database"}))
    .route("/leaderboard/position", get(user_position).post(|| async {"This does NOT support POST requests"}))
}