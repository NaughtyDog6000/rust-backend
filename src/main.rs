#![allow(unused)]

use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

use axum::{
    Router,
    routing::{get, post},
    response::Html,
};


#[tokio::main]
async fn main() {
    println!("intialising");

    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
    .route("/",
         get(|| async { Html("Hello <b>World!!</b>") } ),
    )
    .layer(cors);


    // -- create  server on socket/address 

    let address = SocketAddr::from(([127,0,0,1], 8080));
    println!("Listenting on {address}\n");
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server!");

    // end of server creation
}

