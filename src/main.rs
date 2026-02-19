mod models;
mod database;

use crate::models::todo::Todo;
use crate::database::connection::connect_db;

#[tokio::main]
async fn main() {
    let db = connect_db().await;
    println!("Database connected {:?}",db);
}
