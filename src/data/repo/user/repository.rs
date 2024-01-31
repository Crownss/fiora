use crate::common::errors::Res;
use crate::interactor::user::model::{User, UserWoPw};
use crate::interactor::user::IUserRepository;
use std::fmt::Display;

use super::entity::Req;

pub struct UserRepo {
    user_repo: super::UserDataStore,
}

impl UserRepo {
    pub fn new(user_repo: super::UserDataStore) -> Self {
        Self { user_repo }
    }
}

#[async_trait::async_trait]
impl IUserRepository for UserRepo {
    async fn create_user(&self, user: &User) -> Res<()> {
        self.user_repo.create_user(user).await?;
        Ok(())
    }
    async fn list_user(&self, req: Req) -> Res<(Vec<UserWoPw>, i64)> {
        let res = self.user_repo.list_user(req).await?;
        Ok((res.0.into_iter().map(UserWoPw::from).collect(), res.1))
    }

    // async fn get_user_by<T: Display + std::marker::Sync + tokio_postgres::types::ToSql + std::marker::Send>(&self, param: T) -> Res<UserWoPw>{
    //     let res = self.user_repo.get_user_by(param).await?;
    //     let user = UserWoPw{
    //         id: res.id,
    //         borrowedBookId: res.borrowed_book_id,
    //         firstName: res.first_name,
    //         lastName: res.last_name,
    //         email: res.email,
    //         username: res.username,
    //         createdTime: res.createdtime,
    //         updatedTime: res.updatedtime,
    //     };
    //     Ok(user)
    // }
}
