use axum::{Extension, Json, Router, routing::{get}, http::StatusCode};
use regex::Regex;
use serde::Deserialize;
use sqlx::{pool, PgPool};
use bcrypt::{hash, DEFAULT_COST,};

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
    //parse into user struct
    let SignupUser {username,email,password} = request;
    let hashpass: String = hash(password, DEFAULT_COST).unwrap();

    println!("created a user; username: {username}, email: {email}, password hash: {hashpass} ");
    //verify validity of password, username etc 

    //check that a user with the email & username doesnt already exist

    //add user to database 

    // should add a random delay so that you cannot do some funky stuff to see if a user exists
    // or password funky stuff

    //return success code 200
    StatusCode::OK
}

