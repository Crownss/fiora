use chrono::{DateTime, NaiveDateTime};
use uuid::Uuid;

#[derive(Debug, serde::Serialize)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub createdtime: NaiveDateTime,
    pub updatedtime: NaiveDateTime,
}
