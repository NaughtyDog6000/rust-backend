use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::utils::get_timestamp;
// -- TABLES --

#[derive(Deserialize, sqlx::FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub epoch_signup_time: i64,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct Score {
    pub id: i64,
    pub user_id: i64,
    pub epoch_upload_time: i64,
    pub epoch_game_start_time: i64,
    pub epoch_game_end_time: i64,
    pub score: i32,
    pub game_mode: String,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Token {
    pub id: i64,
    pub user_id: i64,
    pub epoch_expiry_date: i64,
    pub token: String,
    pub creation_timestamp: NaiveDateTime,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct FriendRequest {
    pub id: i64,
    pub sender_id: i64,
    pub receiver_id: i64,
    pub creation_timestamp: NaiveDateTime,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct FriendRecord {
    pub id: i64,
    pub sender_id: i64,
    pub receiver_id: i64,
    pub creation_timestamp: NaiveDateTime,
    pub acceptance_timestamp: NaiveDateTime,
}

// -- END OF TABLES --

// -- enums --

#[derive(Debug, Serialize)]
// #[derive(Debug, Serialize, PartialEq, Eq)]
pub enum RelationshipStatusEnum {
    Friends(FriendRecord),
    UserRequested(FriendRequest),
    TargetRequested(FriendRequest),
    Unrelated,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum VisibilityEnum {
    Private,
    FriendsOnly,
    Public,
}

impl ToString for VisibilityEnum {
    fn to_string(&self) -> String {
        match self {
            VisibilityEnum::Public => String::from("Public"),
            VisibilityEnum::FriendsOnly => String::from("Friends"),
            VisibilityEnum::Private => String::from("Private"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum GamemodeEnum {
    Default,
    Harcore,
}

impl ToString for GamemodeEnum {
    fn to_string(&self) -> String {
        match self {
            GamemodeEnum::Default => String::from("Default"),
            GamemodeEnum::Harcore => String::from("Hardcore"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum OrderByEnum {
    Score,
    MostRecent,
}

impl ToString for OrderByEnum {
    fn to_string(&self) -> String {
        match self {
            OrderByEnum::MostRecent => String::from("scores.epoch_upload_time"),
            OrderByEnum::Score => String::from("scores.score"),
        }
    }
}

// -- END OF enums --

pub fn build_user(
    id: i64,
    username: String,
    email: String,
    password_hash: String,
    epoch_signup_time: Option<i64>,
) -> User {
    let timestamp: i64 = epoch_signup_time.unwrap_or(get_timestamp());
    // if None is passed it gets the current timestamp, else it uses the passed val

    User {
        id,
        username,
        email,
        password_hash: password_hash,
        epoch_signup_time: timestamp,
    }
}
