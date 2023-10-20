use crate::data::infra::psql::TheClient;
pub mod entity;
pub mod data_store;
pub mod repository;
use bb8::Pool;
use bb8_postgres::{tokio_postgres::NoTls, PostgresConnectionManager};

pub struct BookDataStore {
    the_client: Pool<PostgresConnectionManager<NoTls>>,
}