use std::sync::Arc;
use tokio_postgres::Client;
use crate::data::infra::psql::TheClient;
pub mod data_store;
pub mod entity;
pub mod repository;

pub struct UserDataStore {
    the_client: Arc<Client>,
}
