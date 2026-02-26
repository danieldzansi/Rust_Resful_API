use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTodoDto {
    pub description: String,
}

#[derive(Deserialize)]
pub struct UpdateTodo {
    pub description: String,
    pub completed: bool,
}


#[derive(Deserialize)]
pub struct CreateUser {
    pub email :String ,
    pub passord : String ,
}