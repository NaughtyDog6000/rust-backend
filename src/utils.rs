use std::{string, time::{SystemTime, UNIX_EPOCH}};
use chrono::{NaiveDateTime, Utc};
use rand::{distributions::Alphanumeric, Rng};
use regex::Regex;

use crate::structs::{build_user, User, Token, FriendRequest, FriendRecord, FriendStatusEnum, RelationshipRecordEnum};
use log::{warn, info, error};
use serde_json::json;
use sqlx::{pool, PgPool, database::HasValueRef, Error,};
use axum::{Extension, Json, http::StatusCode, headers::Expires};

// -- Timestamp/Datetime --
pub fn get_datetime_utc() -> NaiveDateTime {
    return Utc::now().naive_utc();
}

pub fn get_timestamp() -> i64 {
    let now = SystemTime::now();
    let time_since_epoch = now.duration_since(UNIX_EPOCH).expect("time did a fucky wucky");
    // println!("new signup at: {}", time_since_epoch.as_secs());
    time_since_epoch.as_secs() as i64
}

// -- END Timestamp/Datetime END --

//gets the user from the database when given one of the unuique identifiers (prefering id)
pub async fn get_user(
    pool: &PgPool,
    id: Option<i64>,
    username: Option<String>,
    token: Option<String>
    ) -> Result<User, String> {

    let res: Result<User, sqlx::Error>;

    if id.is_some() {
        res = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(&id.unwrap())
        .fetch_one(pool).await;

    } else if token.is_some() {
        let token_req = sqlx::query_as::<_,Token>("SELECT * FROM tokens
                                WHERE token = $1; 
        ")
        .bind(token.unwrap())
        .fetch_one(pool).await;

        
        
        if token_req.is_err() { return Err(String::from("Token could not be found in database or database encountered an error")); }
        let token_struct: Token  = token_req.unwrap();

        res = sqlx::query_as::<_,User>("SELECT * FROM users WHERE id = $1"
        )
        .bind(token_struct.user_id)
        .fetch_one(pool).await;

    } else if username.is_some() {
        res = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE username = $1"
        )
        .bind(&username.unwrap())
        .fetch_one(pool).await;
    
    } else {
        return Err(String::from("nothing provided to get_user (all params are none)"));
    }




    if res.is_err() {
        println!("{:?}", res);
        return Err(String::from("Failed to fetch user"));
    }

    let user: User = res.unwrap();
    return Ok(user); 
}


// -- Token Creation/Validation --

pub async fn create_session_token(
    pool: &PgPool,
    user: User,
    expires_hours: Option<i32>
    ) -> Result<String, String> {

    const LENGTH: usize = 30; //length of the token 
    const SECONDS_IN_DAY: i64 = 86400;
    let epoch_expiry_timestamp: i64;


    // if the expiry time is not defined set it to 1 day
    if expires_hours.is_none()
    {
        epoch_expiry_timestamp = get_timestamp() + SECONDS_IN_DAY;
    } else {
        epoch_expiry_timestamp = get_timestamp() + (expires_hours.unwrap() as i64 * 3600);
    }

    
    // generate random token
    let token: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(LENGTH)
    .map(char::from)
    .collect();

    // check that the token doesnt already exist (there is like no chance of this even happening)
    let response = sqlx::query("SELECT * FROM tokens
                WHERE token = $1").bind(&token).execute(pool).await;

    if response.is_err() 
    {
        return Err(String::from("token generation failed (the chances of this happening is 1 in 34^30 (8 Quattuordecillion) or i fucked up)"));
    }

    let insrt_rsp = sqlx::query("INSERT INTO tokens (user_id, epoch_expiry_date, token)
                VALUES ($1,$2,$3);
    ")
    .bind(user.id)
    .bind(epoch_expiry_timestamp)
    .bind(&token)
    .execute(pool).await;


    info!("{:?}", insrt_rsp);
    if insrt_rsp.is_err() { return Err(String::from("token database insert failure")); }


    return Ok(token);
}

pub async fn check_token(pool: &PgPool, token: String) -> bool {
    
    // check that it is a regularly non-expired & valid token
    let response = sqlx::query_as::<_, Token>("SELECT * FROM tokens
                                WHERE token = $1 AND epoch_expiry_date > $2; 
    ")
    .bind(token)
    .bind(get_timestamp())
    .fetch_one(pool)
    .await;

    if response.is_err()
    {
        error!("token check failed");
        return false
    }
    let token = response.unwrap();
    println!("{:?}", token);

    return true

}

// -- END Token Creation/Validation END --


// -- Regex Checking/Validations --

// Change this to only accept regular characters and symbols and nothing stpid
pub fn check_password_regex(
    password: &String
) -> bool {
    let reg: Regex = Regex::new(r"^[0-9A-Za-z_.]+$").unwrap();
    if reg.is_match(password) {return true;}
    return false;
}

pub fn check_username_regex(
    password: &String
) -> bool {
    let reg: Regex = Regex::new(r"^[0-9A-Za-z_.]+$").unwrap();
    if reg.is_match(password) {return true;}
    return false;
}

// -- END Regex Checking/Validations END  --


// -- Friends/Friend Requests --

#[derive(Debug)]
pub struct FriendStatus {
    pub status: FriendStatusEnum,
    pub request: Option<FriendRequest>,
    pub record: Option<FriendRecord>,
}

pub async fn get_friend_status(
    pool: &PgPool,
    user_id: i64, 
    target_id: i64
    ) -> Result<FriendStatus, String> {
        
        // make sure that the two users are not the same :')
        if user_id == target_id {
            return Err(String::from("User requested the is the same as the one requesting it"));
        }

        // query the Friend Request table for a friend request between the two users
        let mut response = sqlx::query_as::<_,FriendRequest>("
        SELECT * FROM friend_requests
        WHERE (sender_id = $1 AND receiver_id = $2) OR (sender_id = $2 AND receiver_id = $1)
        ")
        .bind(user_id)
        .bind(target_id)
        .fetch_optional(pool)
        .await;

        //error handling
        if response.is_err() {
            error!("{:?}", response.unwrap());
            return Err(String::from("An error occured in the response from the Friend Request table"));
        }

        // unwrapping as there is no errors
        let opt_request = response.unwrap();

        //if there is a friend request, return that 
        if opt_request.is_some() {
            let opt_request = opt_request.unwrap();
            // if the sender id is the user who sent the request
            if opt_request.sender_id == user_id {
                return Ok(FriendStatus { status: FriendStatusEnum::UserRequested, request: Some(opt_request), record: None });
            } else {
                return Ok(FriendStatus { status: FriendStatusEnum::TargetRequested, request: Some(opt_request), record: None });
            }

        }
        //otherwise query the friends table for a friendship between the two users
        
        let mut response = sqlx::query_as::<_,FriendRecord>("
        SELECT * FROM friends
        WHERE (sender_id = $1 AND receiver_id = $2) OR (sender_id = $2 AND receiver_id = $1)
        ")
        .bind(user_id)
        .bind(target_id)
        .fetch_optional(pool)
        .await;

        //error handling
        if response.is_err() {
            error!("{:?}", response.unwrap_err());
            return Err(String::from("An error was returned from the database on the Friends table query"))
        }

        //unwrap as no err
        let opt_record = response.unwrap();
        
        //if there is a friendship then return that, else return the fact that there is no relation between the users
        if opt_record.is_some() {
            return Ok( FriendStatus { status: FriendStatusEnum::Friends, request: None, record: Some(opt_record.unwrap()) });
        }

        return Ok( FriendStatus { status: FriendStatusEnum::Unrelated, request: None, record: None });
    }
        

    /// returns the vec of friends of the user passed 
    pub async fn get_friends(
        pool: &PgPool,        
        user_id: i64
    ) -> Result<Vec<FriendRecord>, String> {

        let response = sqlx::query_as::<_,FriendRecord>("
        SELECT *
        FROM friends
        WHERE sender_id = $1 OR receiver_id = $1
        ")
        .bind(user_id)
        .fetch_all(pool)
        .await;

        if response.is_err() {
            let error = response.unwrap_err().to_string();
            return Err(error);
        }
            
        let friends = response.unwrap();
        return Ok(friends);
    }
    
    /// returns all the friend 
    pub async fn get_outgoing_friend_requests(
        user_id: i64
    ) -> Result<Vec<FriendRequest>, String> {
        todo!();
        // return Err(String::from("NOT COMPLETE"));
    }

    pub async fn get_incoming_friend_requests(
        user_id: i64
    ) -> Result<Vec<FriendRequest>, String> {
        todo!();
        // return Err(String::from("NOT COMPLETE"));
    }
        
    /// returns a tuple containing the vecs of the relationships (requested, requestee or friends)
    pub async fn get_all_relationships(
        user_id: i64
    ) -> Result<Vec<RelationshipRecordEnum>, String> {
        todo!();
        // return Err(String::from("NOT COMPLETE"));
    }
    
    
    // -- ADDING AND REMOVING FRIENDS --
    

    ///**THIS FUNCTION DOES NOT VERIFY THE VALIDITY OF THE USERS PASSED**<br>
    /// Returns the status of the relationship between the users after the actions have been taken<br>
    /// if an error occurs it is returned as a string
    pub async fn add_or_accept_friend(
        pool: &PgPool,
        user_id: i64, 
        target_id: i64

    ) -> Result<FriendStatusEnum, String> {
        
        let status = get_friend_status(pool, user_id, target_id).await;
        // if no relationship exists, send a friend request
        if status.is_err() {
            let status = status.unwrap_err();
            error!("an error was returned from get friend status: {}", status);
            return Err(status);
        }
        
        match status.unwrap().status {
            // if the users have no relation, send a friend request
            FriendStatusEnum::Unrelated => {
                let response = sqlx::query("
                INSERT INTO friend_requests (sender_id, receiver_id)
                VALUES ($1, $2)
                ")
                .bind(user_id)
                .bind(target_id)
                .execute(pool)
                .await;

                info!("response: {:?}", response);

                return Ok(FriendStatusEnum::UserRequested);
            },
            // if there is a friend request from the target user accept the friend request and send to the users friends table
            FriendStatusEnum::TargetRequested => {
                // UPDATE TO BE A TRANSACTION _________________________________________________
                // todo!();
                let response: Result<sqlx::postgres::PgQueryResult, Error> = sqlx::query("
                DELETE FROM friend_requests
                WHERE sender_id = $1 AND receiver_id = $2
                ")
                .bind(user_id)
                .bind(target_id)
                .execute(pool)
                .await;

                if response.is_err() {
                    return Err(response.unwrap_err().to_string());
                }
                let resp = response.unwrap();
                info!("{:?}", resp);

                let response = sqlx::query("
                INSERT INTO friends (sender_id, receiver_id, creation_timestamp)
	                SELECT sender_id, receiver_id, creation_timestamp
	                FROM friend_requests
	                WHERE sender_id = $1 AND receiver_id = $2;
                ")
                .bind(user_id)
                .bind(target_id)
                .execute(pool)
                .await;

                if response.is_err() {
                    return Err(response.unwrap_err().to_string());
                }
                let resp = response.unwrap();
                info!("{:?}", resp);


                return Ok(FriendStatusEnum::Friends);
            },
            // if they are already friends... why are you trying to add them?
            FriendStatusEnum::Friends => {
                return Ok(FriendStatusEnum::Friends);
            },
            // if the user has already sent a friend request, ... there is nothing to do.
            FriendStatusEnum::UserRequested => {
                return Ok(FriendStatusEnum::UserRequested);
            }
            FriendStatusEnum::Error => {
                return Err(String::from("an error occured"));
            }

        }
    }

    ///**THIS FUNCTION DOES NOT VERIFY THE VALIDITY OF THE USERS PASSED**<br>
    ///if an error occurs it is returned as a string
    pub async fn remove_or_cancel_friend(
        pool: &PgPool,
        user_id: i64, 
        target_id: i64
    ) -> Result<(), String> {
        todo!();

        let response = sqlx::query("
        DELETE FROM friends
        WHERE (sender_id = $1 AND receiver_id = $2) OR (sender_id = $2 AND receiver_id = $1)
        ")
        .bind(user_id)
        .bind(target_id)
        .execute(pool)
        .await;
    
        if response.is_err() {
            println!("{}", response.unwrap_err());
            return  Err(response.unwrap_err().to_string());
        }

        return Ok(());
    }

// -- END Friends/Friend Requests END --
