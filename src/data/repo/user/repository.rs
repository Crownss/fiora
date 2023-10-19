use crate::common::errors::Res;
use crate::domain::user::user_model::User;
use crate::domain::user::IUserRepository;

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
        let res = self.user_repo.create_user(user).await?;
        Ok(())
    }
    async fn list_user(&self) -> Res<Vec<User>> {
        let res = self.user_repo.list_user().await?;
        Ok(res.into_iter().map(User::from).collect())
    }
}
