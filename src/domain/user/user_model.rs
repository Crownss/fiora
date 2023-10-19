use chrono::{DateTime, Local};
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub createdtime: DateTime<Local>,
    pub updatedtime: DateTime<Local>,
}
