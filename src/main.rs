#![allow(unused)]

// modules (other rust scripts used in the project)
mod models;
mod structs;
mod utils;

use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use sqlx::{postgres::PgPoolOptions, error::BoxDynError};

use axum::{
    extract::Extension,
    Router,
    routing::{get, post},
    response::Html,
};

use dotenv;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    println!("intialising");

    // load environment variables from the .env file 
    dotenv::dotenv().ok();
    let envkey = "DB";
    let dbconstring = dotenv::var(envkey).unwrap();
    // println!("connection string: {}", dbconstring);

    let cors = CorsLayer::new().allow_origin(Any);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&dbconstring)
        .await
        .expect("unable to connect to the Database :(");


    // -- End Of Config --

    // -- migrations ig --
    sqlx::migrate!("./migrations").run(&pool).await?;
    // -- end of migrations --



    let app = Router::new()
    .route("/", get(|| async { Html("Hello <b>World!!</b>") } ))
    // .route("/signup", post(models::signup::create_user()))
    .merge(models::signup::router())
    .merge(models::signin::router())
    .layer(cors)
    .layer(Extension(pool));


    // -- create  server on socket/address 

    let address = SocketAddr::from(([127,0,0,1], 8080));
    println!("Listenting on {address}\n");
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server!");

    // end of server creation

    Ok(())
}

