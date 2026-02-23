use sea_orm::{ActiveModelTrait, Set, DatabaseConnection};
use uuid::Uuid;
use chrono::Utc;

use crate::entities::todos;
use crate::dto::todo_dto::CreateTodoDto;
use crate::dto::todo_dto::UpdateTodo;

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

    pub async fn get_all(
        db :&DatabaseConnection,
    ) ->Result<Vec<todos::Model>,sea_orm::DbErr>{
        use sea_orm::EntityTrait;

        todos::Entity::find().all(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        payload: UpdateTodo,
        id: Uuid,
    ) -> Result<todos::Model, sea_orm::DbErr> {
        use sea_orm::EntityTrait;
        if let Some(todo) = todos::Entity::find_by_id(id).one(db).await? {
            let mut active_todo: todos::ActiveModel = todo.into();
            active_todo.description = Set(payload.description);
            active_todo.completed = Set(payload.completed);
            active_todo.update(db).await
        } else {
            Err(sea_orm::DbErr::RecordNotFound("Todo not found".into()))
        }

    }
}



