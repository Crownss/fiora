use crate::common::errors::Res;
use crate::common::responses::DefaultResponse;
use crate::data::repo::user::repository::UserRepo;
use crate::interactor::auth::intr::AuthService;
use crate::interactor::auth::model::ReqRegister;
use actix_web::http::header::ContentType;
use actix_web::web::{Data, Json};
use actix_web::HttpResponse;
use std::sync::Arc;

pub async fn register(
    auth_service: Data<Arc<AuthService<UserRepo>>>,
    req: Json<ReqRegister>,
) -> Res<HttpResponse> {
    let mut resp: DefaultResponse<&str> = DefaultResponse {
        status: "OK".to_string(),
        message: "success".to_string(),
        data: None,
    };
    match auth_service.register(req.0).await {
        Ok(_) => {}
        Err(_) => {
            resp = DefaultResponse::default();
        }
    };
    Ok(HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(resp))
}
