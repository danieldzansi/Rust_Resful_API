use axum::{extract::State, Json};
use sea_orm::DatabaseConnection;

use crate::dto::todo_dto::CreateTodoDto;
use crate::service::todo_service::TodoService;

pub async fn create_todo(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateTodoDto>,
) -> Json<String> {
    match TodoService::create_todo(&db, payload).await {
        Ok(todo) => Json(format!("Todo created : {}", todo.description)),
        Err(e) => Json(format!("Error : {}", e)),
    }
}