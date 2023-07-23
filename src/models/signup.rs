use std::{string, time::Duration};

use axum::{Extension, Json, Router, routing::get, http::StatusCode, };
use log::{warn, error};
use rand::Rng;
use regex::Regex;
use serde::Deserialize;
use sqlx::{pool, PgPool, postgres::PgAdvisoryLockKey};
use bcrypt::{hash, DEFAULT_COST,};

use crate::structs::{build_user, User, get_timestamp};

pub fn router() -> Router {
    Router::new().route("/signup",
    get(|| async {"This does NOT support get requests"}).post(create_user)
    )
}

// static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9A-Za-z_]+$").unwrap());


#[derive(Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct SignupUser {
    username: String,
    email: String,
    password: String
}

pub async fn create_user(
    Extension(pool): Extension<PgPool>,
    Json(request): Json<SignupUser>

) -> StatusCode {
    //parse into temporary struct
    let SignupUser {username,email,password} = request;

    //verify validity of password, username etc

        //check characters used 
    let regex: Regex = Regex::new(r"^[0-9A-Za-z_]+$").unwrap();
    if regex.is_match(&username) {
        warn!("password: {username}, is a vaild username (check by regex)");
    } else {
        warn!("password: {username}, is not a valid username (regex)"); //never prints even when 401 is returned???
        return StatusCode::BAD_REQUEST;
    }

        //check length
        if password.len() <= 7 {
            println!("username valid");
            return StatusCode::BAD_REQUEST;
        }

    //hash the password so no plain text storage (im not the government)
    let hashpass: String = hash(&password, DEFAULT_COST).unwrap();
    println!("created a user; username: {username}, email: {email}, password hash: {hashpass} ");

    //check that a user with the email & username doesnt already exist
    let exists: bool = check_user_exists(&username, &pool).await;

    match exists {
        true => {
            println!("user with username already exists, canceling creation");
            return StatusCode::BAD_REQUEST
        }
        _ => println!("user with that name doesn't exist, finishing creation"),
    }

    

    //add user to database 
    //ToDo: return the id, timestamp etc to be able to get the full user struct
    sqlx::query(
        "INSERT INTO users (username, email, password_hash, epoch_signup_time)
            VALUES ($1, $2, $3, $4);"
    )
    .bind(&username)
    .bind(email)
    .bind(hashpass)
    .bind(get_timestamp())
    .execute(&pool).await;

   
    // should add a random delay so that you cannot do some funky stuff to see if a user exists
    // or password funky stuff
    let sleepy_time = rand::thread_rng().gen_range(Duration::from_millis(100)..=Duration::from_millis(500));
    tokio::time::sleep(sleepy_time).await;

    //return success code 200
    StatusCode::OK
}

pub async fn check_user_exists(username: &String, pool: &PgPool) -> bool {
    let query_res = sqlx::query(
        "SELECT id, epoch_signup_time FROM users WHERE username = $1"
    )
    .bind(&username)
    .fetch_optional(pool).await;

    if query_res.is_err() {
        error!("query returned an error");
        return false;
    }

    if query_res.unwrap().is_some()
    {
        return true;
    } else {
        return false;
    }
}
