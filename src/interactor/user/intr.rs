use crate::common::errors::{CustomError, Res};
use crate::common::responses::{DefaultResponse, ResWCount};
use crate::data::repo::user::entity::Req;
use crate::interactor::user::model::{User, UserWoPw};
use bcrypt::{hash, DEFAULT_COST};
use chrono::Local;
use tracing::error;
use uuid::Uuid;

use super::model;

pub struct UserService<R>
where
    R: super::IUserRepository,
{
    user_repo: R,
}

impl<R> UserService<R>
where
    R: super::IUserRepository,
{
    pub fn new(user_repo: R) -> Self {
        Self { user_repo }
    }

    pub async fn create_user(&self, user: User) -> Res<()> {
        let mut new_user = user;
        if new_user.password != new_user.confPassword {
            error!("password and confPassword not same");
            ()
        }
        let passwordhash = hash(new_user.password, DEFAULT_COST).unwrap();
        let timenow = Local::now().naive_local();
        new_user.id = Uuid::new_v4();
        new_user.borrowedBookId = Uuid::nil();
        new_user.createdTime = timenow;
        new_user.updatedTime = timenow;
        new_user.password = passwordhash;
        self.user_repo.create_user(&new_user).await?;
        Ok(())
    }
    pub async fn list_user(
        &self,
        req: super::model::LimitReq,
    ) -> DefaultResponse<ResWCount<Vec<model::UserWoPw>>> {
        let limit = Req {
            limit: req.limit as i64,
            page: req.page as i64,
        };
        let users = self.user_repo.list_user(limit).await;
        let mut totaling = ResWCount {
            total: 0,
            items: Vec::new(),
        };
        let mut resp = DefaultResponse {
            status: "ERROR".to_string(),
            message: String::from(""),
            data: None,
        };
        match users {
            Ok(res) => {
                totaling.items = res.0;
                totaling.total = res.1 as u64;
                resp.status = "OK".to_string();
                resp.message = "success!".to_string();
                resp.data = Some(totaling);
                resp
            }
            Err(err) => {
                resp.message = err.to_string();
                resp
            }
        }
    }
}
