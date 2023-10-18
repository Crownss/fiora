use chrono::{DateTime, Utc};
use uuid::Uuid;
use tokio_postgres::types::Type;

#[derive(Debug)]
pub struct UserEntity {
    id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    username: String,
    password: String,
    created_at: DateTime<Utc>,
}
