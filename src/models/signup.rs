use std::{string, time::Duration};

use stopwatch::Stopwatch;
use axum::{Extension, Json, Router, routing::get, http::StatusCode, };
use log::{warn, error, info};
use rand::Rng;
use regex::Regex;
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::{pool, PgPool, postgres::PgAdvisoryLockKey};
use bcrypt::{hash, DEFAULT_COST,};

use crate::{structs::{build_user, User, get_timestamp}, utils::check_password_regex};

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

) -> (StatusCode, Json<Value>) {
    //parse into temporary struct
    // let sw = Stopwatch::start_new();
    let SignupUser {username,email,password} = request;

    //verify validity of password, username etc

    
    

    if check_password_regex(&password) {
        warn!("password: {password}, is a vaild username (check by regex)");
    } else {
        warn!("password: {password}, is not a valid username (regex)"); //never prints even when 401 is returned???
        return (StatusCode::BAD_REQUEST, Json(json!("invalid username (un-allowed characters)")));
    }

    // println!("regex & parse took {}ms", sw.elapsed_ms());
    

        //check length
        if password.len() <= 7 {
            println!("username valid");
            return (StatusCode::BAD_REQUEST, Json(json!("password too short")));
        }

    // println!("length check took {}ms", sw.elapsed_ms());


    //hash the password so no plain text storage (im not the government)
    let hashpass: String = hash(&password, 8).unwrap();
    // println!("hash took {}ms", sw.elapsed_ms());
    

    //check that a user with the email & username doesnt already exist
    let exists: bool = check_user_exists(&username, &pool).await;
    // println!("check user exists took {}ms", sw.elapsed_ms());
    

    match exists {
        true => {
            println!("user with username already exists, canceling creation");
            return (StatusCode::BAD_REQUEST, Json(json!({
                "signup": false,
                "details": "user with that username already exists",
                "timestamp": get_timestamp()
        
            })))
        }
        _ => println!("user with that name doesn't exist, finishing creation"),
    }

    let timeestamp: i64 = get_timestamp();
    // println!("timestamp & match user exists took {}ms", sw.elapsed_ms());


    //add user to database 
    //ToDo: return the id, timestamp etc to be able to get the full user struct
    sqlx::query(
        "INSERT INTO users (username, email, password_hash, epoch_signup_time)
            VALUES ($1, $2, $3, $4);"
    )
    .bind(&username)
    .bind(&email)
    .bind(&hashpass)
    .bind(&timeestamp)
    .execute(&pool).await;

    // println!("insert qry took {}ms", sw.elapsed_ms());


    info!("created a user; username: {username}, email: {email}, password hash: {hashpass} ");

   
    // should add a random delay so that you cannot do some funky stuff to see if a user exists
    // or password funky stuff
    // let sleepy_time = rand::thread_rng().gen_range(Duration::from_millis(100)..=Duration::from_millis(500));
    // tokio::time::sleep(sleepy_time).await;

    //return success code 200
    (StatusCode::OK, Json(json!({
        "signup": true,
        "response": "Success",
        "username": username,
        "timestamp": timeestamp

    })))
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
