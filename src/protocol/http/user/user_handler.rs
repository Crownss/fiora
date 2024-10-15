use crate::common::errors::Res;
use crate::common::responses::DefaultResponse;
use crate::data::repo::user::repository::UserRepo;
use crate::interactor::user::intr::UserService;
use crate::interactor::user::model::{GetUserByReq, LimitReq, User, UserWoPw};
use actix_web::http::header::{self, ContentType};
use actix_web::web::{get, Data, Header, Json, Query};
use actix_web::{get, HttpRequest, HttpResponse, Responder, Result};
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
        Err(_) => {
            resp.status = "ERROR".to_string();
            resp.message = "something went wrong!".to_string();
        }
    };
    Ok(HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(resp))
}

pub async fn get_user_by(
    user_service: Data<Arc<UserService<UserRepo>>>,
    req: HttpRequest,
) -> Res<HttpResponse> {
    let getheader = GetUserByReq {
        firstName: format!(
            "{}",
            if let Some(gethead) = req.headers().get("firstName") {
                gethead.to_str().unwrap_or_else(|_| "")
            } else {
                ""
            }
        ),
        lastName: format!(
            "{}",
            if let Some(gethead) = req.headers().get("lastName") {
                gethead.to_str().unwrap_or_else(|_| "")
            } else {
                ""
            }
        ),
        email: format!(
            "{}",
            if let Some(gethead) = req.headers().get("email") {
                gethead.to_str().unwrap_or_else(|_| "")
            } else {
                ""
            }
        ),
        username: format!(
            "{}",
            if let Some(gethead) = req.headers().get("username") {
                gethead.to_str().unwrap_or_else(|_| "")
            } else {
                ""
            }
        ),
    };
    let get_user = user_service.get_user_by(getheader).await;
    Ok(HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(get_user))
}
