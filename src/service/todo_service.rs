use sea_orm::DatabaseConnection;
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
}