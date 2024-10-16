use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct UserEntity {
    pub id: Uuid,
    pub auth_id: Uuid,
    pub full_name: String,
    pub address: String,
    pub gender: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
#[derive(Debug, FromRow)]
pub struct AuthEntity {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct ReqGetUserBy {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}
#[derive(Debug, FromRow)]
pub struct ResGetUserBy {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub address: String,
    pub gender: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for ReqGetUserBy {
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            username: "".to_string(),
            email: "".to_string(),
        }
    }
}

impl Default for ResGetUserBy {
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            username: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
            full_name: "".to_string(),
            address: "".to_string(),
            gender: "".to_string(),
            status: "".to_string(),
            created_at: NaiveDateTime::default(),
            updated_at: NaiveDateTime::default(),
        }
    }
}

impl Default for UserEntity {
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            auth_id: Uuid::nil(),
            full_name: "".to_string(),
            address: "".to_string(),
            gender: "".to_string(),
            status: "".to_string(),
            created_at: NaiveDateTime::default(),
            updated_at: NaiveDateTime::default(),
        }
    }
}
impl Default for AuthEntity {
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            username: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
            created_at: NaiveDateTime::default(),
            updated_at: NaiveDateTime::default(),
        }
    }
}
