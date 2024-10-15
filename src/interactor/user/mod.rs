use async_trait::async_trait;

use crate::{
    common::errors::Res,
    data::repo::user::entity::{Req, ReqFilter},
};
use crate::data::repo::book::entity::UpdateReq;

pub mod intr;
pub mod model;

#[async_trait]
pub trait IUserRepository {
    async fn create_user(&self, user: &model::User) -> Res<()>;
    async fn list_user(&self, req: Req) -> Res<(Vec<model::UserWoPw>, i64)>;
    async fn get_user_by(&self, mut req: ReqFilter) -> Res<model::UserWoPw>;
}
