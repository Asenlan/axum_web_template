use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    // #[serde(skip)]
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub age: i32,
}

impl User {
    pub fn new(id: i32, name: String, email: String, password: String, age: i32) -> Self {
        Self {
            id,
            name,
            email,
            password,
            age,
        }
    }
}
