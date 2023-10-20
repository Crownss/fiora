use std::sync::Arc;
use tokio_postgres::Client;
use crate::data::infra::psql::TheClient;
use bb8::Pool;
use bb8_postgres::{tokio_postgres::{NoTls}, PostgresConnectionManager};
pub mod data_store;
pub mod entity;
pub mod repository;

pub struct UserDataStore {
    the_client: Pool<PostgresConnectionManager<NoTls>>,
}
