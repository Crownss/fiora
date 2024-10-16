use std::env;

use super::model::{ClaimJwt, ReqLogin, ReqRegister, RespLogin};
use crate::{
    common::{errors::Res, responses::DefaultResponse},
    data::repo::user::{
        entity::{ReqGetUserBy, ResGetUserBy},
        repository,
    },
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use tracing::error;
use uuid::Uuid;

pub struct AuthService<R>
/*
   this is auth service which is need user repo for register or login stand for "interface user repository"
   This repository is a bridge between the service and something done on the database.
*/
where
    R: repository::IUserRepository,
{
    user_repo: R,
}

impl<R> AuthService<R>
where
    R: repository::IUserRepository,
{
    pub fn new(user_repo: R) -> Self {
        Self { user_repo }
    }
    pub async fn register(&self, req: ReqRegister) -> Res<()> {
        if req.password != req.confirmPassword {
            error!("password and confPassword not same");
            return Ok(());
        }
        self.user_repo.create_user(req).await?;
        Ok(())
    }
    pub async fn login(&self, req: ReqLogin) -> Res<DefaultResponse<RespLogin>> {
        let mut resp = DefaultResponse {
            status: "OK".to_string(),
            message: "success".to_string(),
            data: None,
        };
        let mut respLogin = RespLogin {
            id: Uuid::nil(),
            fullName: "".to_string(),
            username: None,
            email: None,
            token: "".to_string(),
        };
        let mut req_repo = ReqGetUserBy::default();
        let mut res = ResGetUserBy::default();
        if let Some(email) = req.email {
            req_repo.email = email.clone();
            res = self.user_repo.get_user_by(req_repo).await?;
            respLogin.email = Some(email);
        } else if let Some(username) = req.username {
            req_repo.username = username.clone();
            res = self.user_repo.get_user_by(req_repo).await?;
            respLogin.username = Some(username);
        }
        if !bcrypt::verify(req.password, res.password.as_str()).unwrap() {
            error!("password from request and password from db doesn't match");
            return Ok(DefaultResponse::default());
        }
        let exp = Utc::now() + Duration::days(7);
        let claimjwt = ClaimJwt {
            id: res.id,
            fullName: res.full_name,
            exp: exp.timestamp(),
        };
        let token = encode(
            &Header::default(),
            &claimjwt,
            &EncodingKey::from_secret(env::var("psql.username").unwrap().as_ref()),
        )?;
        respLogin.token = token;
        resp.data = Some(respLogin);
        Ok(resp)
    }
}
