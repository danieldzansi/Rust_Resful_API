use axum::{extract::State, Json};
use sea_orm::DatabaseConnection;

use crate::dto::todo_dto::CreateTodoDto;
use crate::service::todo_service::TodoService;
use crate::entities::todos::Model;

pub async fn create_todo(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateTodoDto>,
) -> Json<String> {
    match TodoService::create_todo(&db, payload).await {
        Ok(todo) => Json(format!("Todo created : {}", todo.description)),
        Err(e) => Json(format!("Error : {}", e)),
    }
}

pub async fn get_all_todos(
    State(db): State<DatabaseConnection>,
) -> Json<Vec<Model>> {
    match TodoService::get_all_todos(&db).await{
        Ok(todos)=>Json(todos),
        Err(e)=> Json(vec![]),
    }
}