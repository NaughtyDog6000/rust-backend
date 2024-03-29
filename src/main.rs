#![allow(unused)]

// modules (other rust scripts used in the project)
mod errors;
mod models;
mod structs;
mod utils;

use serde_json::{json, Value};
use sqlx::{error::BoxDynError, postgres::PgPoolOptions};
use std::{
    fs::{self, *},
    io::Write,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::Path,
};
use structs::VisibilityEnum;
use tower_http::cors::{AllowMethods, Any, CorsLayer};

use axum::{
    extract::{DefaultBodyLimit, Extension},
    http::{HeaderMap, Method, StatusCode},
    response::Html,
    routing::{get, post},
    Json, Router,
};

use dotenv;
use log::{debug, error, info, trace, warn};
use log4rs;
use std::error::Error;

use crate::utils::get_timestamp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("intialising");

    //Logging file
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    // trace!("detailed tracing info on your mothers maiden name");
    // debug!("debug info");
    // info!("relevant general info");
    // warn!("this may be bad, you should take a look");
    // error!("guys you whould come take a look, something *Really* bads just happened");

    let args: Vec<String> = std::env::args().collect();

    // load environment variables from the .env file
    dotenv::dotenv().ok();
    let envkey = "DATABASE_URL";

    let dbconstring = dotenv::var(envkey).unwrap();

    // GIANT MESS
    let port = dotenv::var("PORT");
    let port_str = port.unwrap();
    let port = port_str.parse::<u16>().unwrap();
    // println!("{:?}", port);

    let addr = dotenv::var("HOST_ADDRESS").unwrap();
    let address_parts_str: Vec<&str> = addr.split(".").collect();
    let address_parts: Result<Vec<u8>, _> = address_parts_str.iter().map(|x| x.parse()).collect();
    let address_parts: Vec<u8> = address_parts.unwrap();
    // println!("{:?}", address_parts);
    // GIANT MESS

    // -- Get the admin key --
    let admin_key: String = dotenv::var("ADMINKEY").expect("could not get the admin key");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(Any);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&dbconstring)
        .await
        .expect("unable to connect to the Database :(");

    // -- End Of Config --

    // -- migrations ig --
    if (args.len() > 1) {
        if (&args[1] == "migrate") {
            warn!("MIGRATING");
            sqlx::migrate!("./migrations").run(&pool).await?;
        }
    }

    // -- end of migrations --

    let app = Router::new()
        .fallback(page_not_found)
        .route("/", get(root_get).post(root_post))
        .route("/ping", get(ping_get).post(ping_post))
        .merge(models::leaderboard::router())
        .merge(models::account_managment::router())
        .merge(models::profile_managment::router())
        .merge(models::_old_routes::router())
        .merge(models::utilities::router())
        .layer(cors) //-- for testing
        .layer(Extension(admin_key))
        .layer(Extension(pool))
        .layer(DefaultBodyLimit::max(1048576)); //1MB max size

    pub async fn page_not_found() -> axum::response::Html<&'static str> {
        include_str!("./html/404.html").into()
    }

    // -- create  server on socket/address

    let address = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(
            address_parts[0],
            address_parts[1],
            address_parts[2],
            address_parts[3],
        )),
        port,
    );
    println!("Listenting on {address}\n");
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server!");

    // end of server creation

    Ok(())
}

pub async fn root_post(headers: HeaderMap) -> (StatusCode, Json<Value>) {
    println!("{:?}", headers);
    return (
        StatusCode::OK,
        Json(json!({
            "response": "HELLO",
            "method_sent": "POST",
            "Time": get_timestamp()
        })),
    );
}

pub async fn root_get(headers: HeaderMap) -> (StatusCode, Json<Value>) {
    println!("{:?}", headers);
    return (
        StatusCode::OK,
        Json(json!({
            "response": "HELLO",
            "method_sent": "GET",
            "Time": get_timestamp()
        })),
    );
}

pub async fn ping_post(headers: HeaderMap) -> (StatusCode, Json<Value>) {
    println!("{:?}", headers);
    return (
        StatusCode::OK,
        Json(json!({
            "response": "pong POST",
        })),
    );
}

pub async fn ping_get(headers: HeaderMap) -> (StatusCode, Json<Value>) {
    println!("{:?}", headers);
    return (
        StatusCode::OK,
        Json(json!({
            "response": "pong GET"
        })),
    );
}
