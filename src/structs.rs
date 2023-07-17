use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};



#[derive(Deserialize, sqlx::FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub epoch_signup_time: i64,
}

#[derive(Deserialize, Serialize)]
pub struct JTWCustomClaims {
    pub id: i64,
    pub username: String,
}


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



pub fn get_timestamp() -> i64 {
    let now = SystemTime::now();
    let time_since_epoch = now.duration_since(UNIX_EPOCH).expect("time did a fucky wucky");
    // println!("new signup at: {}", time_since_epoch.as_secs());
    time_since_epoch.as_secs() as i64
}