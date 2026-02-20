
use axum::{routing::{get, post}, Router};
use crate::handlers::todo_handler::{create_todo,get_all_todos};
use sea_orm::DatabaseConnection;

pub fn create_routes() -> Router<DatabaseConnection> {
    Router::new()
        .route("/", get(home))
        .route("/todos", post(create_todo))
        .route("/all/todos", get(get_all_todos))
}

async fn home() -> &'static str {
    "Rust server is running"
}

