use axum::{Router, routing::{get, post}};

use self::old_leaderboard::leaderboard_old;


pub mod old_leaderboard;

pub fn router() -> Router {
    Router::new()
    .route("/old/scores/global",get(leaderboard_old).post(|| async {"This does NOT support POST requests"}))
}