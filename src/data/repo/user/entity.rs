use chrono::{DateTime, NaiveDateTime};
// use tokio_postgres::types::FromSql;
use crate::interactor::user::user_model::User;
use uuid::Uuid;

#[derive(Debug)]
pub struct UserEntity {
    pub id: Uuid,
    pub borrowed_book_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub createdtime: NaiveDateTime,
    pub updatedtime: NaiveDateTime,
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
            createdtime: NaiveDateTime::default(),
            updatedtime: NaiveDateTime::default(),
        }
    }
}

impl From<UserEntity> for User {
    fn from(entity: UserEntity) -> Self {
        Self {
            id: entity.id,
            borrowedBookId: entity.borrowed_book_id,
            firstName: entity.first_name,
            lastName: entity.last_name,
            email: entity.email,
            username: entity.username,
            password: entity.password,
            confPassword: "".to_string(),
            createdTime: entity.createdtime,
            updatedTime: entity.updatedtime,
        }
    }
}
