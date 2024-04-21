use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub user_name: String,
    pub email: String,
}

impl User {
    pub fn new(payload: CreateUser) -> Self {
        Self {
            id: 1,
            user_name: payload.user_name,
            email: payload.email,
        }
    }
}
