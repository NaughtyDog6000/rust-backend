use std::string;

use axum::{Extension, Json, Router, routing::get, http::StatusCode};
use regex::Regex;
use serde::{Deserialize, __private::ser::FlatMapSerializeStruct};
use sqlx::{pool, PgPool, postgres::PgAdvisoryLockKey};
use bcrypt::{hash, DEFAULT_COST,};
use rand::Rng;
use std::time::Duration;
use crate::structs::{build_user, User};

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
    let regex: Regex = Regex::new(r"^[0-9A-Za-z_]+$").unwrap();
    if regex.is_match(&password) {
        println!("password: {password}, is a vaild password (check by regex)");
    } else {
        print!("password: {password}, is not a valid password (regex)"); //never prints even when 401 is returned???
        return StatusCode::UNAUTHORIZED;
    }

    //hash the password so no plain text storage (im not the government)
    let hashpass: String = hash(&password, DEFAULT_COST).unwrap();
    println!("created a user; username: {username}, email: {email}, password hash: {hashpass} ");

    //parse into the permanenet user struct
    let mut user: crate::structs::User = build_user(username, email, hashpass) ;

    //check that a user with the email & username doesnt already exist
    let exists: bool = check_user_exists(&user.username, &pool).await;

    match exists {
        true => {
            println!("user with username already exists, canceling creation");
            return StatusCode::BAD_REQUEST
        }
        _ => println!("user with that name doesn't exist, finishing creation"),
    }

    //add user to database 

    sqlx::query(
        "INSERT INTO users (username, email, password_hash, epoch_signup_time)
            VALUES ($1, $2, $3, $4);"
    )
    .bind(&user.username)
    .bind(user.email)
    .bind(user.password_hash)
    .bind(user.epoch_signup_time)
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
    .fetch_optional(pool)
    .await;

    let mut res = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = $1"
    )
    .bind(&username)
    .fetch_one(pool)
    .await;

    // let resp = sqlx::query_as!(User,
    // "
    // SELECT id, epoch_signup_time
    // FROM users
    // WHERE username = ?
    // ",
    // organization
    // )
    // .fetch_all(&pool)
    // .await?;
    if res.is_err() {
        return false;
    }
    println!("response: {:?}", res);
    
    // println!("{:?}", query_res);
    
        // TB WORKED ON ----!_!_!_!_!_!_!_

    return true;
}
