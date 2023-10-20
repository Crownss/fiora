use bcrypt::{DEFAULT_COST, hash};
use tracing::error;
use crate::common::errors::{Res, CustomError};
use crate::interactor::user::user_model::User;
use uuid::Uuid;
use chrono::{DateTime, Local, NaiveDateTime, NaiveDate, NaiveTime};

pub struct UserService<R>
    where
        R: super::IUserRepository,
{
    user_repo: R,
}

impl<R> UserService<R>
    where R: super::IUserRepository
{
    pub fn new(user_repo: R) -> Self {
        Self { user_repo }
    }

    pub async fn create_user(&self, user: User) -> Res<()> {
        let mut new_user = user;
        if new_user.password != new_user.confPassword{
            // Err("password not same")
            error!("password and confPassword not same");
            CustomError::HttpError("password and confPassword not same".to_string());
            ()
        }
        let passwordhash = hash(new_user.password, DEFAULT_COST).unwrap();
        // let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();
        // let t = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();
        let timenow = Local::now().naive_local();;
        new_user.id = Uuid::new_v4();
        new_user.borrowedBookId = Uuid::nil();
        new_user.createdTime = timenow;
        new_user.updatedTime = timenow;
        new_user.password = passwordhash;
        self.user_repo.create_user(&new_user).await?;
        Ok(())
    }
    pub async fn list_user(&self) -> Res<Vec<User>> {
        let users = self.user_repo.list_user().await?;
        Ok(users)
    }
}