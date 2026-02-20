use sea_orm::{ActiveModelTrait, Set, DatabaseConnection};
use uuid::Uuid;
use chrono::Utc;

use crate::entities::todos;
use crate::dto::todo_dto::CreateTodoDto;

pub struct TodoRepositoy;

impl TodoRepositoy {
    pub async fn create(
        db: &DatabaseConnection,
        dto: CreateTodoDto,
    ) -> Result<todos::Model, sea_orm::DbErr> {
        let todo = todos::ActiveModel {
            id: Set(Uuid::new_v4()),
            description: Set(dto.description),
            completed: Set(false),
            created_at: Set(Utc::now().into()),
            ..Default::default()
        };
        todo.insert(db).await
    }
}