use axum::{Router, routing::{get, post}};


pub mod check_user_exists;

pub fn router() -> Router {
    Router::new()
    .route("/check_user", get(check_user_exists::get_user_exists))
}