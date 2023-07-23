use std::string;

use crate::structs::{build_user, User, get_timestamp, JTWCustomClaims};
use log::{warn, info, error};
use serde_json::json;
use sqlx::{pool, PgPool, database::HasValueRef, Error,};
use axum::{Extension, Json, http::StatusCode};
use jwt_simple::prelude::*;



//gets the user from the database when given one of the unuique identifiers (prefering id)
pub async fn get_user(
    Extension(pool): Extension<PgPool>,
    id: Option<i64>,
    username: Option<String>
    ) -> Result<User, String> {

    let res: Result<User, sqlx::Error>;

    if id.is_some() {
        res = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(&id.unwrap())
        .fetch_one(&pool).await;
    } else {
        res = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE username = $1"
        )
        .bind(&username.unwrap())
        .fetch_one(&pool).await;
    }




    if res.is_err() {
        return Err(String::from("Failed to fetch user"));
    }
    let user: User = res.unwrap();
    return Ok(user); 
}

pub fn create_jwt(key:HS256Key, user: User, expires_seconds: u64) -> String {

    let custom_claims = JTWCustomClaims {
        id: user.id,
        username: user.username,
        creation_time: get_timestamp(),
    };
    let claims = Claims::with_custom_claims(custom_claims, Duration::from_secs(expires_seconds));
    // let claims = Claims::create(Duration::from_secs(expires_seconds));
    let token: String = key.authenticate(claims).expect("could not authenticate/generate a JWT");


    return token;
}

pub async fn check_token(Extension(pool): Extension<PgPool>, key:HS256Key, token: String) -> Result<User, String> {
    
    // check that it is a regularly non-expired & valid token
    let mut claims = key.verify_token::<JTWCustomClaims>(&token, Default::default());
    match &claims {
        Err(error) => {
            warn!("bad token use attempted");
            return Err(String::from("token invalid"));
        }
        _ => { }
    }

    // check that the user hasnt signed out (invalidated all tokens) after this was created
    error!("invalidating tokens not complete");

    let claims = claims.unwrap();

    let mut expire_before = sqlx::query("SELECT epoch_invalidate_tokens FROM users WHERE id = $1;").bind(&claims.custom.id).execute(&pool).await;
    match expire_before {
        Ok(_) => {}
        Err(_) => { error!("checking user token expire before date");  return  Err(String::from("select fail")); }
    }

    let expire_before = expire_before.unwrap();
    println!("{:?}", expire_before);

    return get_user(Extension(pool), Some(claims.custom.id), None).await;

    // return Err(String::from("error occured in token check"));
}



pub fn get_scores_default(
    Extension(pool): Extension<PgPool>,
) -> Result<User, String>  {
    


    warn!("THIS FUNCTION IS INCOMPLETE");
    Err(String::from("INCOMPLETE FUNCTION"))
}