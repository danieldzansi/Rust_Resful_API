use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTodoDto {
    pub description: String,
}