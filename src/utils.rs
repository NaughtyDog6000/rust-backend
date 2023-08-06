use std::string;
use rand::{distributions::Alphanumeric, Rng};
use regex::Regex;

use crate::structs::{build_user, User, get_timestamp, Token};
use log::{warn, info, error};
use serde_json::json;
use sqlx::{pool, PgPool, database::HasValueRef, Error,};
use axum::{Extension, Json, http::StatusCode, headers::Expires};



//gets the user from the database when given one of the unuique identifiers (prefering id)
pub async fn get_user(
    pool: &PgPool,
    id: Option<i64>,
    username: Option<String>,
    token: Option<String>
    ) -> Result<User, String> {

    let res: Result<User, sqlx::Error>;

    if id.is_some() {
        res = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(&id.unwrap())
        .fetch_one(pool).await;

    } else if token.is_some() {
        let token_req = sqlx::query_as::<_,Token>("SELECT * FROM tokens
                                WHERE token = $1; 
        ")
        .bind(token.unwrap())
        .fetch_one(pool).await;

        
        
        if token_req.is_err() { return Err(String::from("Token could not be found in database or database encountered an error")); }
        let token_struct: Token  = token_req.unwrap();

        res = sqlx::query_as::<_,User>("SELECT * FROM users WHERE id = $1"
        )
        .bind(token_struct.user_id)
        .fetch_one(pool).await;

    } else if username.is_some() {
        res = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE username = $1"
        )
        .bind(&username.unwrap())
        .fetch_one(pool).await;
    
    } else {
        return Err(String::from("nothing provided to get_user (all params are none)"));
    }




    if res.is_err() {
        println!("{:?}", res);
        return Err(String::from("Failed to fetch user"));
    }

    let user: User = res.unwrap();
    return Ok(user); 
}

pub async fn create_session_token(
    pool: &PgPool,
    user: User,
    expires_hours: Option<i32>
    ) -> Result<String, String> {

    const LENGTH: usize = 30; //length of the token 
    const SECONDS_IN_DAY: i64 = 86400;
    let epoch_expiry_timestamp: i64;


    // if the expiry time is not defined set it to 1 day
    if expires_hours.is_none()
    {
        epoch_expiry_timestamp = get_timestamp() + SECONDS_IN_DAY;
    } else {
        epoch_expiry_timestamp = get_timestamp() + (expires_hours.unwrap() as i64 * 3600);
    }

    
    // generate random token
    let token: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(LENGTH)
    .map(char::from)
    .collect();

    // check that the token doesnt already exist (there is like no chance of this even happening)
    let response = sqlx::query("SELECT * FROM tokens
                WHERE token = $1").bind(&token).execute(pool).await;

    if response.is_err() 
    {
        return Err(String::from("token generation failed (the chances of this happening is 1 in 34^30 (8 Quattuordecillion) or i fucked up)"));
    }

    let insrt_rsp = sqlx::query("INSERT INTO tokens (user_id, epoch_expiry_date, token)
                VALUES ($1,$2,$3);
    ")
    .bind(user.id)
    .bind(epoch_expiry_timestamp)
    .bind(&token)
    .execute(pool).await;


    info!("{:?}", insrt_rsp);
    if insrt_rsp.is_err() { return Err(String::from("token database insert failure")); }


    return Ok(token);
}

pub async fn check_token(pool: &PgPool, token: String) -> bool {
    
    // check that it is a regularly non-expired & valid token
    let response = sqlx::query_as::<_, Token>("SELECT * FROM tokens
                                WHERE token = $1 AND epoch_expiry_date > $2; 
    ")
    .bind(token)
    .bind(get_timestamp())
    .fetch_one(pool)
    .await;

    if response.is_err()
    {
        error!("token check failed");
        return false
    }
    let token = response.unwrap();
    println!("{:?}", token);

    return true

    // return Err(String::from("error occured in token check"));
}

// Change this to only accept regular characters and symbols and nothing stpid
pub fn check_password_regex(
    password: &String
) -> bool {
    let reg: Regex = Regex::new(r"^[0-9A-Za-z_.]+$").unwrap();
    if reg.is_match(password) {return true;}
    return false;
}

pub fn check_username_regex(
    password: &String
) -> bool {
    let reg: Regex = Regex::new(r"^[0-9A-Za-z_.]+$").unwrap();
    if reg.is_match(password) {return true;}
    return false;
}