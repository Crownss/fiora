use async_trait::async_trait;

use crate::common::errors::Res;

pub mod user_model;
pub mod user_service;

#[async_trait]
pub trait IUserRepository {
    async fn create_user(&self, user: &user_model::User) -> Res<()>;
    async fn list_user(&self) -> Res<Vec<user_model::User>>;
    // async fn get_user(&self, id: &Uuid) -> Res<user_model::User>;
}