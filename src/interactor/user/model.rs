use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
pub struct UserWoPw {
    pub id: Uuid,
    pub borrowedBookId: Uuid,
    pub firstName: String,
    pub lastName: String,
    pub email: String,
    pub username: String,
    pub createdTime: NaiveDateTime,
    pub updatedTime: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct LimitReq {
    pub limit: u32,
    pub page: u32,
}

#[derive(Debug, Deserialize)]
pub struct GetUserByReq {
    pub firstName: String,
    pub lastName: String,
    pub email: String,
    pub username: String,
    pub borrowedBookId: String,
}
