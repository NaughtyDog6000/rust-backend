use axum::{
    routing::{get, post},
    Router,
};

pub mod check_user;

pub fn router() -> Router {
    Router::new().route(
        "/check_user",
        get(check_user::check_user).post(check_user::check_user),
    )
}
