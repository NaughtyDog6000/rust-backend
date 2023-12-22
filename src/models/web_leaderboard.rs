use std::fmt::Debug;

use axum::{
    Extension, Json, Router,
    routing::{get, post},
    http::{StatusCode, HeaderMap},
    response::{IntoResponse, Response},
    extract::Query
};

use sqlx::{pool, PgPool};
use serde_json::{json, Value};