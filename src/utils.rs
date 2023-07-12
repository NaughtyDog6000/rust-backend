use crate::structs::{build_user, User, get_timestamp};
use sqlx::{pool, PgPool, database::HasValueRef,};
use axum::Extension;




//gets the user from the database when given one of the unuique identifiers (prefering id)
pub async fn get_user(
    Extension(pool): Extension<PgPool>,
    id: Option<i64>,
    username: Option<String>
    ) -> Result<User, String> {

    let res: Result<User, sqlx::Error>;

    if id.is_some() {
        res = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(&id.unwrap())
        .fetch_one(&pool).await;
    } else {
        res = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE username = $1"
        )
        .bind(&username.unwrap())
        .fetch_one(&pool).await;
    }




    if res.is_err() {
        return Err(String::from("Failed to fetch user"));
    }
    let user: User = res.unwrap();
    return Ok(user); 
}
