use crate::service::todo_service::UserService;
use axum::{extract::State, Json};
use axum::extract::Path;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::dto::todo_dto::CreateTodoDto;
use crate::dto::todo_dto::CreateUser;
use crate::dto::todo_dto::UpdateTodo;
use crate::service::todo_service::TodoService;
use crate::entities::todos::Model as TodoModel;
use crate::entities::users::Model as UserModel;
use serde_json::json;

pub async fn create_todo(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateTodoDto>,
) -> axum::Json<serde_json::Value> {
    match TodoService::create_todo(&db, payload).await {
        Ok(todo) => Json(json!({
            "message": "Todo created",
            "id": todo.id,
            "description": todo.description,
            "completed": todo.completed,
            "created_at": todo.created_at
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

pub async fn get_all_todos(
    State(db): State<DatabaseConnection>,
) -> axum::Json<serde_json::Value> {
    match TodoService::get_all_todos(&db).await {
        Ok(todos) => Json(json!({ "todos": todos })),
        Err(_e) => Json(json!({ "todos": [] })),
    }
}

pub async fn update_todo(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTodo>,
) -> Result<axum::Json<serde_json::Value>, String> {
    match TodoService::update_todo(&db, id, payload).await {
        Ok(todo) => Ok(Json(json!({
            "message": "Todo updated",
            "id": todo.id,
            "description": todo.description,
            "completed": todo.completed,
            "created_at": todo.created_at
        }))),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn delete_todo(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<axum::Json<serde_json::Value>, String> {
    match TodoService::delete_todo(&db, id).await {
        Ok(_) => Ok(Json(json!({ "message": "Todo deleted successfully" }))),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn create_user (
    State(db): State<DatabaseConnection>,
    Json(payload):Json<CreateUser>,
) -> axum::Json<serde_json::Value> {
    match UserService::create_user(&db, payload).await {
        Ok(user) => Json(serde_json::json!({
            "message": "User created",
            "email": user.email,
            "password": user.password
        })),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}
