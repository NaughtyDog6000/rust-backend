use serde::Deserialize;
use std::time::{Duration, SystemTime, UNIX_EPOCH};


#[derive(Deserialize, sqlx::FromRow, Debug)]
pub struct User {
    
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub epoch_signup_time: i64,

}

pub fn build_user(
    username: String,
    email: String,
    password_hash: String,
    
) -> User {
    User {
        username,
        email,
        password_hash: password_hash,
        epoch_signup_time: get_timestamp(),
    }

}

fn get_timestamp() -> i64 {
    let now = SystemTime::now();
    let time_since_epoch = now.duration_since(UNIX_EPOCH).expect("time did a fucky wucky");
    // println!("new signup at: {}", time_since_epoch.as_secs());
    time_since_epoch.as_secs() as i64
}