// this route is for adding/
use std::string;

use axum::{
    Extension, Json, Router,
    routing::{get, post},
    http::StatusCode,
    response::{IntoResponse, Response},
    extract::Query
};

use bcrypt::hash;
use chrono::NaiveDate;
use log::{warn, info, trace, error};
use serde::Deserialize;
use sqlx::{pool, PgPool};
use serde_json::{json, Value};

use crate::{utils::{check_token, get_user, check_password_regex}, structs::User};

const HASHING_ROUNDS: u32 = 8;

pub fn router() -> Router {
    Router::new()
    .route("/account/update/date_of_birth", get(|| async {"to update send token & a date string formatted: YY-MM-DD or YYYY-M-D etc"}).post(add_date_of_birth))
    .route("/account/update/password", get(|| async {"to update password send token & new password in body of a POST request"}).post(update_password)
    )
}

#[derive(Deserialize)]
pub struct UpdateDOBParams {
    token: String,
    date: String,
    
}

#[derive(Deserialize)]
pub struct UpdatePasswordRequestParams {
    token: String,
    password: String,
}


pub async fn add_date_of_birth(
    Extension(pool): Extension<PgPool>,
    Json(request): Json<UpdateDOBParams>
) -> (StatusCode, Json<Value>) {
    
    // -- check the toekn --
    let user = get_user(&pool, None, None, Some(request.token)).await;
    if user.is_err() 
    {
        return (StatusCode::BAD_REQUEST, Json(json!("invalid token")));
    }
    let user = user.unwrap();


    // -- parse the input date string --

    let date_option = request.date.parse::<NaiveDate>();
    // let date_option = NaiveDate::from_ymd_opt(2023, 7, 24);
    if date_option.is_ok()
    {
        println!("{}", date_option.unwrap());
    } else {
        error!("invalid (impossible) date entered ");
        return (StatusCode::BAD_REQUEST, Json(json!("DATE OF BIRTH IMPOSSIBLE")));

    }
    
    let response = sqlx::query("UPDATE users
                SET date_of_birth = $1 
                WHERE id = $2
    ")
    .bind(date_option.unwrap())
    .bind(user.id)
    .execute(&pool).await;

    println!("response from query: {:?}", response);

    (StatusCode::OK, Json(json!("DATE OF BIRTH UPDATED")))
}

pub async fn update_password(
    Extension(pool): Extension<PgPool>,
    Json(request): Json<UpdatePasswordRequestParams>
) -> (StatusCode, Json<Value>) {
    // -- check token --
    let user = get_user(&pool, None, None, Some(request.token)).await;


    if user.is_err() {return (StatusCode::BAD_REQUEST, Json(json!({"response": "BAD TOKEN"})));}
    let user: User = user.unwrap();

    // -- check new password is valid --
    if !check_password_regex(&request.password) {return (StatusCode::BAD_REQUEST, Json(json!({"response": "BAD TOKEN"})));}
    // -- hash the password --
    let password_hash: String = hash(request.password, HASHING_ROUNDS).unwrap();

    // -- upload to db --
    let response_pass_update = sqlx::query("
    UPDATE users
    SET password_hash = $1
    WHERE id = $2")
    .bind(password_hash)
    .bind(user.id)
    .execute(&pool)
    .await;
    println!("response: {:?}", response_pass_update);
    warn!("PASSWORD UPDATING NOT COMPLETE");
    // -- signout all --

    return (StatusCode::OK, Json(json!({"response": "PSSWD UPDATE NOT FIN"})))
}