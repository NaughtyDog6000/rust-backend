use axum::{
    routing::{get, post},
    Router,
};

pub mod achievements;
pub mod friend_managment;
pub mod user_account_profile;

pub fn router() -> Router {
    Router::new()
        .merge(friend_managment::router())
        .route(
            "/profile",
            get(user_account_profile::get_profile)
                .post(|| async { "This does NOT support POST requests" }),
        )
        .route(
            "/admin/achievements/create",
            get(achievements::get_admin_create_achievement)
                .post(achievements::admin_create_achievement),
        )
        .route("/achievements/unlock", post(|| async { "WIP" }))
}
