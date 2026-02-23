use axum::{extract::State, Json};
use axum::extract::Path;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::dto::todo_dto::CreateTodoDto;
use crate::dto::todo_dto::UpdateTodo;
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

pub async fn update_todo(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Model>, String> {
    match TodoService::update_todo(&db, id, payload).await {
        Ok(todo) => Ok(Json(todo)),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn delete_todo(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<Json<String>, String> {
    match TodoService::delete_todo(&db, id).await {
        Ok(_) => Ok(Json("Todo deleted successfully".to_string())),
        Err(e) => Err(e.to_string()),
    }
}