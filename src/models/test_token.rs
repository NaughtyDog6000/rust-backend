use axum::{Extension, Json, Router, routing::get, http::StatusCode, response::{IntoResponse, Response}};
use jwt_simple::prelude::HS256Key;
use jwt_simple::prelude::*;
use log::{warn, info};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::{pool, PgPool};

use crate::structs::JTWCustomClaims;



pub fn router() -> Router {
    Router::new().route("/test_token",
    get(|| async {"This does NOT support get requests"}).post(test_token)
    )
}

#[derive(Serialize, Deserialize)]
pub struct JWTRequestParams {
    token: String
}


pub async fn test_token(
    Extension(key): Extension<HS256Key>,
    Extension(pool): Extension<PgPool>,
    Json(request): Json<JWTRequestParams>
) -> (StatusCode, Json<Value>) {


    let claims = key.verify_token::<JTWCustomClaims>(&request.token, Default::default());
    match &claims {
        Ok(claims) => {
            info!("successful token use");
            println!("claims: {}", claims.custom.username);
        }
        Err(error) => {
            warn!("bad token use attempted");
            return (StatusCode::BAD_REQUEST, Json(json!("bad token")))
        }
    }
    let claims = claims.unwrap();

    (StatusCode::OK, Json(json!(claims.custom.username)))
}