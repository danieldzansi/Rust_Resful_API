
use axum::{routing::{get, post, put, delete}, Router};
use crate::handlers::todo_handler::{create_todo, get_all_todos, update_todo, delete_todo,create_user};
use sea_orm::DatabaseConnection;

pub fn create_routes() -> Router<DatabaseConnection> {
    Router::new()
        .route("/", get(home))
        .route("/todos", post(create_todo))
        .route("/user", post(create_user))
        .route("/all/todos", get(get_all_todos))
        .route("/todos/{id}", put(update_todo))
        .route("/todos/{id}", delete(delete_todo))
}

async fn home() -> &'static str {
    "Rust server is running"
}

