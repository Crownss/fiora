use crate::common::errors::Res;
use crate::common::responses::DefaultResponse;
use crate::data::repo::user::repository::UserRepo;
use crate::interactor::user::intr::UserService;
use crate::interactor::user::model::{LimitReq, User, UserWoPw};
use actix_web::http::header::{self, ContentType};
use actix_web::web::{Data, Json, Query};
use actix_web::{get, HttpResponse, Responder, Result};
use std::sync::Arc;

pub async fn list_users(
    user_service: Data<Arc<UserService<UserRepo>>>,
    req: Query<LimitReq>,
) -> Res<HttpResponse> {
    let list_user = user_service.list_user(req.0).await;
    Ok(HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(list_user))
}

pub async fn create_users(
    user_service: Data<Arc<UserService<UserRepo>>>,
    req: Json<super::dto::CreateUserRequest>,
) -> Res<HttpResponse> {
    let new_user = User::try_from(req.0)?;
    let mut resp: DefaultResponse<&str> = DefaultResponse {
        status: "OK".to_string(),
        message: "success".to_string(),
        data: None,
    };
    match user_service.create_user(new_user).await {
        Ok(_) => {}
        Err(err) => {
            resp.status = "ERROR".to_string();
            resp.message = "something went wrong!".to_string();
        }
    };
    Ok(HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(resp))
}
