use axum::{Extension, Json, Router, routing::get, http::StatusCode};
use serde::Deserialize;
use sqlx::{pool, PgPool, postgres::PgAdvisoryLockKey};

use crate::{utils::get_user, structs::User};


#[derive(Deserialize)]
pub struct SigninRequestParams {
    username: String,
    email: String,
    password: String
}


pub fn router() -> Router {
    Router::new().route("/signin",
    get(|| async {"This does NOT support get requests"}).post(signin)
    )
}

pub async fn signin (
    Extension(pool): Extension<PgPool>,
    Json(request): Json<SigninRequestParams>
) -> StatusCode {
    //parse the JSON Body of the request
    let SigninRequestParams {username, email, password} = request;
    
    //find the user account with the username
    let user: User = get_user(Extension(pool),None, Some(username)).await.expect("FUCK");
    //chech the hash with the request password

    //return a invalid requst code should any of this fail

    //return a jwt or access token thing idk fuitre me problem

    
    StatusCode::OK
}