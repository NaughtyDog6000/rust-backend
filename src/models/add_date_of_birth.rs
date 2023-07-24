//This route is for disabling all current tokens associated with the account 
// (therefore signing out all current devices)

use std::string;

use axum::{
    Extension, Json, Router,
    routing::{get, post},
    http::StatusCode,
    response::{IntoResponse, Response},
    extract::Query
};

use chrono::NaiveDate;
use jwt_simple::prelude::HS256Key;
use log::{warn, info, trace, error};
use serde::Deserialize;
use sqlx::{pool, PgPool};
use serde_json::{json, Value};

use crate::utils::check_token;


pub fn router() -> Router {
    Router::new().route("/profile/update/date_of_birth",
        get(|| async {"to update send token & a date string formatted: YY-MM-DD or YYYY-M-D etc"})
        .post(add_date_of_birth)
    )
}

#[derive(Deserialize)]
pub struct UpdateDOBParams {
    token: String,
    date: String,
    
}


pub async fn add_date_of_birth(
    Extension(key): Extension<HS256Key>,
    Extension(pool): Extension<PgPool>,
    Json(request): Json<UpdateDOBParams>
) -> (StatusCode, Json<Value>) {

    // -- check the toekn --
    let user = check_token(pool.clone(), key, request.token).await;
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