// use sqlx::{Pool , Postgres};
// use dotenvy::dotenv;
// use std::env;

// pub type DbPool = Pool<Postgres>;


// pub async fn connect_db () ->DbPool{
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

//     Pool::<Postgres>::connect(&database_url)
//     .await 
//     .expect("failed to connect to database")
// }

use sea_orm::{Database, DatabaseConnection};
use dotenvy::dotenv;
use std::env;

pub type DbPool = DatabaseConnection;

pub async fn connect_db() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    Database::connect(database_url)
        .await
        .expect("Failed to connect to database")
}