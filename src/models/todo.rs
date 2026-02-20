use uuid::Uuid;
// use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};


pub struct Todo {
    pub id: Uuid,
    pub description: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}
