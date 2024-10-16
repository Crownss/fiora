use chrono::{Utc};
use uuid::Uuid;

use super::entity::{AuthEntity, ReqGetUserBy, ResGetUserBy, UserEntity};
use crate::common::errors::Res;
use crate::interactor::auth::model::ReqRegister;
use async_trait::async_trait;

pub struct UserRepo {
    user_repo: super::UserDataStore,
}

#[async_trait]
pub trait IUserRepository {
    /*
       IUserRepository stand for "interface user repository"
       This repository is a bridge between the service and something done on the database.
    */
    async fn create_user(&self, req: ReqRegister) -> Res<()>;
    async fn get_user_by(&self, req: ReqGetUserBy) -> Res<ResGetUserBy>;
    // async fn list_user(&self, req: Req) -> Res<(Vec<model::UserWoPw>, i64)>;
    // async fn get_user_by(&self, mut req: ReqFilter) -> Res<model::UserWoPw>;
}

impl UserRepo {
    pub fn new(user_repo: super::UserDataStore) -> Self {
        Self { user_repo }
    }
}

#[async_trait]
impl IUserRepository for UserRepo {
    async fn create_user(&self, req: ReqRegister) -> Res<()> {
        let mut tx = self.user_repo.the_client.begin().await?;
        let passwordhash = bcrypt::hash(req.password, bcrypt::DEFAULT_COST).unwrap();
        let timenow = Utc::now().naive_local();
        let req_auth = AuthEntity {
            id: Uuid::new_v4(),
            username: req.username,
            email: req.email,
            password: passwordhash,
            created_at: timenow,
            updated_at: timenow,
        };
        //create auth table first then returning auth_id for create user table//
        let auth_id = self.user_repo.create_auth(&mut tx, &req_auth).await?;
        let req_user = UserEntity {
            id: Uuid::new_v4(),
            auth_id: auth_id,
            full_name: req.fullName,
            address: req.address,
            gender: req.gender,
            status: "active".to_string(),
            created_at: timenow,
            updated_at: timenow,
        };
        self.user_repo.create_user(&mut tx, &req_user).await?;
        tx.commit().await?;
        Ok(())
    }
    async fn get_user_by(&self, req: ReqGetUserBy) -> Res<ResGetUserBy> {
        Ok(self.user_repo.get_user_by(req).await?)
    }
}
