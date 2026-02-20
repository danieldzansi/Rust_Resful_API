mod models;
mod database;
mod routes;

use crate::models::todo::Todo;
use crate::database::connection::connect_db;
use crate::routes::router::create_routes;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let db = connect_db().await;
    println!("Database connected");

    let app = create_routes();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("server runnning on port {:?}",listener);

    axum::serve(listener,app).await.unwrap();

}
