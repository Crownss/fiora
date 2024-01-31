use sqlx::{Pool, Postgres};

pub mod data_store;
pub mod entity;
pub mod repository;

pub struct BookDataStore {
    the_client: Pool<Postgres>,
}
