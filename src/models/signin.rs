use axum::{Extension, Json, Router, routing::get, routing::post, http::StatusCode, response::{IntoResponse, Response}};
use log::info;
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::{pool, PgPool, postgres::PgAdvisoryLockKey};
use bcrypt::{verify, hash, DEFAULT_COST};
use rand::Rng;
use std::time::Duration;

use crate::{utils::{get_user, create_session_token}, structs::User};


#[derive(Deserialize)]
pub struct SigninRequestParams {
    username: Option<String>,
    email: Option<String>,
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
    
    let username: String = username.unwrap(); 

    //find the user account with the username
    let user: User = match get_user(&pool, None, Some(username), None).await {
        Ok(user) => user,
        Err(error) => {
            
            return (StatusCode::BAD_REQUEST, 
                Json(json!({
                     "response": "user does not exist"
                     }))
            );
        } 
    }; 
    
    //chech the hash with the request password
    let pass_correct: bool = verify(&password, &user.password_hash).unwrap();
    info!("Password-Hash comparison: {}", pass_correct);


    // -- Create Token --
    let token = create_session_token(&pool, user, None).await;
    match token.is_err() {
        true => return (StatusCode::BAD_REQUEST, Json(json!({"error in token creation": token}))),
        false => (),
        _ => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!("IDEK how this happened")))
    }

    let token = token.unwrap();
    //add a random delay (even though the chance of anyyone (even alex) abusing the timings to know shit is like 1*10^-69%)
    
    // let sleepy_time = rand::thread_rng().gen_range(Duration::from_millis(100)..=Duration::from_millis(500));
    // tokio::time::sleep(sleepy_time).await;
    





    match pass_correct {
        true => {
            return (StatusCode::OK, Json(json!({
                "token": token
            })));
        }
        false => {    
            return (StatusCode::BAD_REQUEST, Json(json!({
            "ERROR": "PASSWORD OR USERNAME INCORRECT"})));    
        }
    }
    

}