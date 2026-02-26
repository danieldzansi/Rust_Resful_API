use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use sea_orm::DatabaseConnection;
use uuid::Uuid;
use crate::dto::todo_dto::{CreateTodoDto, CreateUser, UpdateTodo};
use crate::service::todo_service::{TodoService, UserService};

pub async fn create_todo(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateTodoDto>,
) -> impl axum::response::IntoResponse {
    match TodoService::create_todo(&db, payload).await {
        Ok(todo) => (StatusCode::CREATED, Json(todo)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn get_all_todos(
    State(db): State<DatabaseConnection>,
) -> impl axum::response::IntoResponse {
    match TodoService::get_all_todos(&db).await {
        Ok(todos) => (StatusCode::OK, Json(todos)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn update_todo(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTodo>,
) -> impl axum::response::IntoResponse {
    match TodoService::update_todo(&db, id, payload).await {
        Ok(todo) => (StatusCode::OK, Json(todo)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn delete_todo(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> impl axum::response::IntoResponse {
    match TodoService::delete_todo(&db, id).await {
        Ok(_) => (StatusCode::NO_CONTENT, ()).into_response(),
        Err(e) => (StatusCode::NOT_FOUND, e.to_string()).into_response(),
    }
}

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateUser>,
) -> impl axum::response::IntoResponse {
    match UserService::create_user(&db, payload).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}