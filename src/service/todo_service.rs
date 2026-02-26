use crate::repository::todo_repo::UserRepository;
use sea_orm::DatabaseConnection;
use uuid::Uuid;
use crate::dto::todo_dto::UpdateTodo;
use crate::dto::todo_dto::CreateUser;
use crate::dto::todo_dto::CreateTodoDto;
use crate::entities::users::Model as UserModel;
use crate::entities::todos::Model as TodoModel;
use crate::repository::todo_repo::TodoRepository;

pub struct TodoService;

pub struct UserService ;

impl TodoService {
    pub async fn create_todo(
        db: &DatabaseConnection,
        dto: CreateTodoDto,
    ) -> Result<TodoModel, sea_orm::DbErr> {
        if dto.description.trim().is_empty() {
            return Err(sea_orm::DbErr::Custom("Description cannot be empty".into()));
        }
        TodoRepository::create(db, dto).await
    }

    pub async fn get_all_todos(
        db: &DatabaseConnection,
    ) -> Result<Vec<TodoModel>, sea_orm::DbErr> {
        TodoRepository::get_all(db).await
    }

    pub async fn update_todo(
        db: &DatabaseConnection,
        id: Uuid,
        payload: UpdateTodo
    ) -> Result<TodoModel, sea_orm::DbErr> {
        if payload.description.trim().is_empty() {
            return Err(sea_orm::DbErr::Custom("Description cannot be empty".into()));
        }
        TodoRepository::update(db, payload, id).await
    }

    pub async fn delete_todo(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<(), sea_orm::DbErr> {
        TodoRepository::delete(db, id).await
    }
}

impl UserService{
    pub async fn create_user(
      db: &DatabaseConnection,
      dto:CreateUser
    ) -> Result<UserModel, sea_orm::DbErr> {
        if dto.email.trim().is_empty() {
            return Err(sea_orm::DbErr::Custom("Email is required".to_string()));
        } else if dto.passord.trim().is_empty() {
            return Err(sea_orm::DbErr::Custom("Password is required".to_string()));
        }
        UserRepository::createuser(db, dto).await
    }

}    