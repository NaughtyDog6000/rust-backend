use serde::{Deserialize};

#[derive(Deserialize, sqlx::FromRow)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,

}