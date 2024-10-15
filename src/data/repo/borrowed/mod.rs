use std::sync::Arc;
use sqlx::{Pool, Postgres};
use crate::data::repo::user::repository::UserRepo;

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