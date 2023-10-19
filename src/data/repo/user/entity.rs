use chrono::{DateTime, Local};
// use tokio_postgres::types::FromSql;
use uuid::Uuid;
use crate::domain::user::user_model::User;

#[derive(Debug)]
pub struct UserEntity {
    pub id: Uuid,
    pub borrowed_book_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub createdtime: DateTime<Local>,
    pub updatedtime: DateTime<Local>,
}

impl Default for UserEntity {
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            borrowed_book_id: Uuid::nil(),
            first_name: "".to_string(),
            last_name: "".to_string(),
            email: "".to_string(),
            username: "".to_string(),
            password: "".to_string(),
            createdtime: DateTime::default(),
            updatedtime: DateTime::default(),
        }
    }
}

impl From<UserEntity> for User {
    fn from(entity: UserEntity) -> Self {
        Self {
            id: entity.id,
            first_name: entity.first_name,
            last_name: entity.last_name,
            email: entity.email,
            username: entity.username,
            password: entity.password,
            createdtime: entity.createdtime,
            updatedtime: entity.updatedtime,
        }
    }
}