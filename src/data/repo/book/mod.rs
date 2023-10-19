use crate::data::infra::psql::TheClient;
pub mod entity;
pub mod data_store;
pub mod repository;

pub struct BookDataStore {
    the_client: TheClient,
}