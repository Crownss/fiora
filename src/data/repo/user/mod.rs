use std::sync::Arc;

use sqlx::{Pool, Postgres};
pub mod data_store;
pub mod entity;
pub mod repository;

pub struct UserDataStore {
    the_client: Arc<Pool<Postgres>>,
}
