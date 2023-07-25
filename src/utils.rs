use std::string;
use rand::{distributions::Alphanumeric, Rng};

use crate::structs::{build_user, User, get_timestamp, Token};
use log::{warn, info, error};
use serde_json::json;
use sqlx::{pool, PgPool, database::HasValueRef, Error,};
use axum::{Extension, Json, http::StatusCode, headers::Expires};



//gets the user from the database when given one of the unuique identifiers (prefering id)
pub async fn get_user(
    pool: &PgPool,
    id: Option<i64>,
    username: Option<String>
    ) -> Result<User, String> {

    let res: Result<User, sqlx::Error>;

    if id.is_some() {
        res = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(&id.unwrap())
        .fetch_one(pool).await;
    } else {
        res = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE username = $1"
        )
        .bind(&username.unwrap())
        .fetch_one(pool).await;
    }




    if res.is_err() {
        return Err(String::from("Failed to fetch user"));
    }
    let user: User = res.unwrap();
    return Ok(user); 
}

pub async fn create_token(
    pool: &PgPool,
    user: User,
    expires_hours: Option<i32>
    ) -> Result<String, String> {

    const LENGTH: usize = 30; //length of the token 

    // if the expiry time is not defined set it to 1 day
    if expires_hours.is_none()
    {
        let expires_hours: i32 = 24;
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

    let insrt_rsp = sqlx::query("INSERT INTO tokens (user_id, epoch_expriry_date)
                VALUES ($1,$2);
    ")
    .bind(user.id)
    .bind(expires_hours)
    .execute(pool);


    return Ok(token);
}

pub async fn check_token(pool: &PgPool, token: String) -> Result<User, String> {
    
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
        return Err(String::from("token check failed"));
    }
    let token = response.unwrap();

    return get_user(pool, Some(token.id), None).await;

    // return Err(String::from("error occured in token check"));
}



pub fn get_scores_default(
    pool: PgPool,
) -> Result<User, String>  {
    


    warn!("THIS FUNCTION IS INCOMPLETE");
    Err(String::from("INCOMPLETE FUNCTION"))
}