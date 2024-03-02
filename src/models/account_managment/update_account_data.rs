// this route is for adding/
use std::string;

use axum::{
    extract::Query,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
};

use bcrypt::hash;
use chrono::NaiveDate;
use log::{error, info, trace, warn};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::{pool, PgPool};

use crate::{
    structs::User,
    utils::{check_password_regex, check_token, get_user},
};

const HASHING_ROUNDS: u32 = 8;

#[derive(Deserialize)]
pub struct UpdateDOBParams {
    date: String,
}

#[derive(Deserialize)]
pub struct UpdatePasswordRequestParams {
    password: String,
}

pub async fn add_date_of_birth(
    Extension(pool): Extension<PgPool>,
    headers: HeaderMap,
    Json(request): Json<UpdateDOBParams>,
) -> (StatusCode, Json<Value>) {
    let auth_token = headers.get("auth");
    if auth_token.is_none() {
        return (
            StatusCode::IM_A_TEAPOT,
            Json(json!({
                "response": "token not present you melon"
            })),
        );
    }
    let auth_token = auth_token.unwrap().to_str().unwrap().to_owned();

    // -- check the toekn --
    let user = get_user(&pool, None, None, Some(auth_token)).await;
    if user.is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"response":"invalid token"})),
        );
    }
    let user = user.unwrap();

    // -- parse the input date string --

    let date_option = request.date.parse::<NaiveDate>();
    // let date_option = NaiveDate::from_ymd_opt(2023, 7, 24);
    if date_option.is_ok() {
        println!("{}", date_option.unwrap());
    } else {
        error!("invalid (impossible) date entered ");
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"response": "DATE OF BIRTH IMPOSSIBLE"})),
        );
    }

    let response = sqlx::query(
        "UPDATE users
                SET date_of_birth = $1 
                WHERE id = $2
    ",
    )
    .bind(date_option.unwrap())
    .bind(user.id)
    .execute(&pool)
    .await;

    println!("response from query: {:?}", response);

    (StatusCode::OK, Json(json!("DATE OF BIRTH UPDATED")))
}

pub async fn update_password(
    Extension(pool): Extension<PgPool>,
    headers: HeaderMap,
    Json(request): Json<UpdatePasswordRequestParams>,
) -> (StatusCode, Json<Value>) {
    // -- get token from headers --
    let auth_token = headers.get("auth");
    if auth_token.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "response": "token not present you melon"
            })),
        );
    }
    let auth_token = auth_token.unwrap().to_str().unwrap().to_owned();

    // -- check token --
    let user = get_user(&pool, None, None, Some(auth_token)).await;

    if user.is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"response": "BAD TOKEN"})),
        );
    }
    let user: User = user.unwrap();

    // -- check new password is valid --
    if !check_password_regex(&request.password) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"response": "BAD TOKEN"})),
        );
    }
    // -- hash the password --
    let password_hash: String = hash(request.password, HASHING_ROUNDS).unwrap();

    // -- upload to db --
    let response_pass_update = sqlx::query(
        "
    UPDATE users
    SET password_hash = $1
    WHERE id = $2",
    )
    .bind(password_hash)
    .bind(user.id)
    .execute(&pool)
    .await;
    println!("response: {:?}", response_pass_update);

    // -- signout all --
    let response = sqlx::query(
        "DELETE FROM tokens
                WHERE user_id = $1",
    )
    .bind(user.id)
    .execute(&pool)
    .await;

    let rows_affected = response.unwrap().rows_affected();

    return (
        StatusCode::OK,
        Json(json!({
            "response": "success",
            "signed_out": rows_affected
        })),
    );
}
