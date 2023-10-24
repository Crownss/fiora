use crate::common::errors::Res;
use crate::common::responses::DefaultResponse;
use crate::data::repo::user::repository::UserRepo;
use crate::interactor::user::user_model::User;
use crate::interactor::user::user_service::UserService;
use actix_web::http::header::{self, ContentType};
use actix_web::web::{Data, Json};
use actix_web::{get, HttpResponse, Responder, Result};
use std::sync::Arc;

pub async fn list_users(user_service: Data<Arc<UserService<UserRepo>>>) -> Res<HttpResponse> {
    let list_user = user_service.list_user().await?;
    let resp: DefaultResponse<Vec<User>> = DefaultResponse {
        status: "0001".to_string(),
        message: "success".to_string(),
        data: Some(list_user),
    };
    // match list_user {
    //     Ok(res) => resp.data = Some(res),
    //     Err(_) => resp.data = None,
    // };
    // Ok(Json(resp))
    Ok(HttpResponse::Ok().insert_header(ContentType::json()).json(resp))
}

pub async fn create_users(
    user_service: Data<Arc<UserService<UserRepo>>>,
    req: Json<super::dto::CreateUserRequest>,
) -> Res<HttpResponse> {
    let new_user = User::try_from(req.0)?;
    let mut resp: DefaultResponse<String> = DefaultResponse {
        status: "0001".to_string(),
        message: "success".to_string(),
        data: None,
    };
    match user_service.create_user(new_user).await {
        Ok(_) => {}
        Err(err) => {
            resp.status = "5000".to_string();
            resp.message = "something went wrong!".to_string();
        }
    };
    Ok(HttpResponse::Ok().insert_header(ContentType::json()).json(resp))
}
