use bcrypt::{DEFAULT_COST, hash};
use crate::common::errors::Res;
use crate::domain::user::user_model::User;
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
        let passwordhash = hash(new_user.password, DEFAULT_COST).unwrap();
        let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();
        let t = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();
        let timenow = NaiveDateTime::new(d, t);
        new_user.id = Uuid::new_v4();
        new_user.createdtime = timenow;
        new_user.updatedtime = timenow;
        new_user.password = passwordhash;
        let user = self.user_repo.create_user(&new_user).await?;
        Ok(())
    }
    pub async fn list_user(&self) -> Res<Vec<User>> {
        let users = self.user_repo.list_user().await?;
        Ok(users)
    }
}