use chrono::{DateTime, NaiveDateTime};
use uuid::Uuid;

#[derive(Debug, serde::Serialize)]
pub struct User {
    pub id: Uuid,
    pub borrowedBookId: Uuid,
    pub firstName: String,
    pub lastName: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub confPassword: String,
    pub createdTime: NaiveDateTime,
    pub updatedTime: NaiveDateTime,
}
