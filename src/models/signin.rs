use axum::{Extension, Json, Router, routing::get, routing::post, http::StatusCode, response::{IntoResponse, Response}};
use log::info;
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::{pool, PgPool, postgres::PgAdvisoryLockKey};
use bcrypt::{verify, hash, DEFAULT_COST};
use rand::Rng;
use std::time::Duration;

use crate::{utils::{get_user, create_token}, structs::User};


#[derive(Deserialize)]
pub struct SigninRequestParams {
    username: String,
    email: String,
    password: String
}




pub fn router() -> Router {
    Router::new().route("/signin",
    post(signin).get(|| async {"This does NOT support get requests"})
    )
}

pub async fn signin (
    Extension(pool): Extension<PgPool>,
    Json(request): Json<SigninRequestParams>
) -> (StatusCode, Json<Value>) {
    //parse the JSON Body of the request
    let SigninRequestParams {username, email, password} = request;
    
    //find the user account with the username
    let user: User = match get_user(&pool, None, Some(username)).await {
        Ok(user) => user,
        Err(error) => {
            
            return (StatusCode::BAD_REQUEST, Json(json!("user does not exist")));
        } 
    }; 
    
    //chech the hash with the request password
    let pass_correct: bool = verify(&password, &user.password_hash).unwrap();
    info!("Password-Hash comparison: {}", pass_correct);


    //add a random delay (even though the chance of anyyone (even alex) abusing the timings to know shit is like 1*10^-69%)
    
    // let sleepy_time = rand::thread_rng().gen_range(Duration::from_millis(100)..=Duration::from_millis(500));
    // tokio::time::sleep(sleepy_time).await;
    


    //return a jwt or access token thing idk fuitre me problem or not if the password is incoreect





    match pass_correct {
        true => {
            return (StatusCode::OK, Json(json!({
                "token": create_token(&pool, user, None).await.unwrap()
            })));
        }
        false => {}
    }
    

    

    (StatusCode::OK, Json(json!({
        "ERROR": "PASSWORD OR USERNAME INCORRECT",
    })))
}