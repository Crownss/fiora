use crate::data::repo::user::repository::UserRepo;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

pub mod data_store;
pub mod entity;
mod repository;

pub struct BorrowedDataStore {
    the_client: Arc<Pool<Postgres>>,
}

pub struct BorrowedRepo {
    repo: BorrowedDataStore,
}

impl BorrowedRepo {
    pub fn new(repo: BorrowedDataStore) -> Self {
        Self { repo }
    }
}
