mod models;
mod database;
mod routes;
mod handlers;
mod repository;
mod service;
mod dto;
mod entities;

use crate::database::connection::connect_db;
use crate::routes::router::create_routes;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {

    let db = connect_db().await;
    println!("Database connected");

    let app = create_routes().with_state(db);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}