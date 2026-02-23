use sea_orm::DatabaseConnection;
use uuid::Uuid;
use crate::dto::todo_dto::UpdateTodo;
use crate::dto::todo_dto::CreateTodoDto;
use crate::entities::todos::Model;
use crate::repository::todo_repo::TodoRepositoy;

pub struct TodoService;

impl TodoService {
    pub async fn create_todo(
        db: &DatabaseConnection,
        dto: CreateTodoDto,
    ) -> Result<Model, sea_orm::DbErr> {
        if dto.description.trim().is_empty() {
            return Err(sea_orm::DbErr::Custom("Description cannot be empty".into()));
        }
        TodoRepositoy::create(db, dto).await
    }

    pub async fn get_all_todos(
        db: &DatabaseConnection,
    ) -> Result<Vec<Model>, sea_orm::DbErr> {
        TodoRepositoy::get_all(db).await
    }

    pub async fn update_todo(
        db: &DatabaseConnection,
        id: Uuid,
        payload: UpdateTodo
    ) -> Result<Model, sea_orm::DbErr> {
        if payload.description.trim().is_empty() {
            return Err(sea_orm::DbErr::Custom("Description cannot be empty".into()));
        }
        TodoRepositoy::update(db, payload, id).await
    }
}