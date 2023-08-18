use axum::{Extension, Json, Router, routing::get, routing::post, http::{StatusCode, HeaderMap}, response::{IntoResponse, Response}, extract::Query};
use log::{warn, info, trace, error};
use serde::Deserialize;
use sqlx::{pool, PgPool};
use serde_json::{json, Value};

use crate::errors::{handle_error, CustomErrors};





pub fn router() -> Router {
    Router::new().route("/admin/achievements/create", get(get_admin_create_achievement).post(admin_create_achievement)
    ).route("/upload", post(upload))
}

pub async fn get_admin_create_achievement() -> axum::response::Html<&'static str> {
    include_str!("../html/create_achievement_form.html").into()
}


async fn admin_create_achievement(
    Extension(pool): Extension<PgPool>,
    Extension(admin_key): Extension<String>,
    headers: HeaderMap,
) -> (StatusCode, Json<Value>) {
    // -- get token from headers -- 
    let auth_token = headers.get("auth");
    if auth_token.is_none() {
        return (StatusCode::IM_A_TEAPOT, Json(json!({
            "response": "token not present you melon"
        })));
    }
    let auth_token: String = auth_token.unwrap().to_str().unwrap().to_owned(); 

    // -- get the username of the person being added from body or query string --
    let requested_username: String;

    return handle_error(CustomErrors::Unimplemented);
}


async fn upload(mut multipart: Multipart) {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of `{}` is {} bytes", name, data.len());
    }
}