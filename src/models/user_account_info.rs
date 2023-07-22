use std::string;

use axum::{
    Extension, Json, Router,
    routing::{get, post},
    http::StatusCode,
    response::{IntoResponse, Response},
    extract::Query
};

use jwt_simple::prelude::HS256Key;
use log::{warn, info, trace, error};
use serde::Deserialize;
use sqlx::{pool, PgPool};
use serde_json::{json, Value};


pub fn router() -> Router {
    Router::new().route("/profile/:id",
        get(|| async {"this is not done"})
        .post(|| async {"This does NOT support POST requests"})
    )
}