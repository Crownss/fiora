use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;
use crate::data::repo::user::entity::UserEntity;

#[derive(Debug, FromRow)]
pub struct BorrowedEntity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub book_id: Uuid,
    pub status: String, //activated || returned
    pub return_date: NaiveDateTime,
    pub createdtime: NaiveDateTime, //borrowed date (never change since user borrowing)
    pub updatedtime: NaiveDateTime,
}

impl Default for BorrowedEntity {
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            user_id: Uuid::nil(),
            book_id: Uuid::nil(),
            status: "".to_string(),
            return_date: NaiveDateTime::default(),
            createdtime: NaiveDateTime::default(),
            updatedtime: NaiveDateTime::default(),
        }
    }
}