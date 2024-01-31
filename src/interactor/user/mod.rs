use async_trait::async_trait;

use crate::{common::errors::Res, data::repo::user::entity::Req};

pub mod intr;
pub mod model;

#[async_trait]
pub trait IUserRepository {
    async fn create_user(&self, user: &model::User) -> Res<()>;
    async fn list_user(&self, req: Req) -> Res<(Vec<model::UserWoPw>, i64)>;
    // async fn get_user_by<T: Display + std::marker::Sync + tokio_postgres::types::ToSql + std::marker::Send>(&self, param: T) -> Res<user_model::UserWoPw>;
}
