// fine I will do the stupid error Enum thing :)

// jk I will not use this for a while

// WAS GOING To:
// taken from this site and modified for my uses 
// https://www.sheshbabu.com/posts/rust-error-handling/

// then I remembered being recomended by Jack to use this:
// https://docs.rs/thiserror/latest/thiserror/


//! This is the eror handling section of the program, all errors that could occur should be handled here

use axum::{Json, http::StatusCode};
use log::warn;
use log4rs::encode::json;
use serde_json::{Value, json};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomErrors {
// #[error("the database encountered an error ({0})")]    
// DatabaseError(String),

#[error(transparent)]
SQLXError(#[from] sqlx::Error),  

#[error("an error involving the token used occured")]
TokenError,

#[error("the user {unknown_user} does not exist")]    
UserDoesNotExist {
    unknown_user: String,
},

#[error("an access token was missing or invalid")]    
BadToken,

#[error("the following params are missing: {missing_params}")]
MissingQueryParams {
    missing_params: String,
},

#[error("the user is requesting themself where that is not possible")]    
RequestingSelf,

#[error("an error occurred ¯\\_(ツ)_/¯")]    
LogicError,

#[error("this part of the API is not complete")]
Unimplemented,
}



/// a function that when passed a "CustomErrors" Enum will return: <br>
/// the appropriate statuscode & the json response of the error, Wrapped into a tupple<br>
/// this can then be directly returned by the calling route
pub fn handle_error(error: CustomErrors) -> (StatusCode, Json<Value>) {
    warn!("{:?}", error);
    
    match error {
        CustomErrors::SQLXError(sqlx_error) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "response": sqlx_error.to_string()
            })))
        },
        CustomErrors::BadToken => {
            return (StatusCode::BAD_REQUEST, Json(json!({
                "response": error.to_string()
            })));
        },
        CustomErrors::TokenError => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "response": error.to_string()
            })));
        },
        CustomErrors::UserDoesNotExist {unknown_user} => {
            return (StatusCode::BAD_REQUEST, Json(json!({
                "response": format!("user: {}, does not exist", unknown_user)
            })));
        },
        CustomErrors::RequestingSelf => {
            return (StatusCode::BAD_REQUEST, Json(json!({
                "response": error.to_string()
            })));
        },
        CustomErrors::LogicError => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "response": error.to_string()
            })));
        },
        CustomErrors::MissingQueryParams { missing_params } => {
            return (StatusCode::BAD_REQUEST, Json(json!({
                "rsponse": format!("the following params are missing: {}", missing_params)
            })));
        },
        CustomErrors::Unimplemented => {
            return (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "response": error.to_string()
            })));
        },
    }
}