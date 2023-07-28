#![allow(unused)]

// modules (other rust scripts used in the project)
mod models;
mod structs;
mod utils;

use std::{fs::{*, self}, net::{SocketAddr, IpAddr, Ipv4Addr}, io::Write, path::Path};
use serde_json::json;
use tower_http::cors::{Any, CorsLayer};
use sqlx::{postgres::PgPoolOptions, error::BoxDynError};

use axum::{
    extract::Extension,
    Router,
    routing::{get, post},
    response::Html, Json,
};

use dotenv;
use std::error::Error;
use log::{trace, debug, info, warn, error};
use log4rs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    println!("intialising");
    
        //Logging file
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
        // trace!("detailed tracing info");
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
    let address_parts_str: Vec<&str> =addr.split(".").collect();
    let address_parts: Result<Vec<u8>,_> = address_parts_str.iter().map(|x| x.parse()).collect();
    let address_parts: Vec<u8> = address_parts.unwrap();
    // println!("{:?}", address_parts);
    // GIANT MESS

    let cors = CorsLayer::new().allow_origin(Any);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&dbconstring)
        .await
        .expect("unable to connect to the Database :(")
        ;

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
    .route("/", get(|| async { Html("Hello <b>GET!!</b>") } ).post(|| async { Html("Hello <b>POST!!</b>") } ))
    .route("/ping", get(|| async { Html("pong GET") } ).post(|| async { Html("PONG POST") } ))
    .merge(models::signup::router())
    .merge(models::signin::router())
    .merge(models::test_token::router())
    .merge(models::leaderboard::router())
    .merge(models::upload_score::router())
    .merge(models::user_account_info::router())
    .merge(models::signout::router())
    .merge(models::update_date_of_birth::router())
    .merge(models::delete_account::router())
    .merge(models::update_password::router())

    .layer(cors)
    .layer(Extension(pool));

    // -- create  server on socket/address 

    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(address_parts[0],address_parts[1],address_parts[2],address_parts[3],)), port);
    println!("Listenting on {address}\n");
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server!");

    // end of server creation

    Ok(())
}