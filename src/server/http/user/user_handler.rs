use crate::common::errors::Res;
use crate::common::responses::DefaultResponse;
use crate::data::repo::user::repository::UserRepo;
use crate::domain::user::user_service::UserService;
use actix_web::web::Json;
use actix_web::{HttpResponse, Responder, get, Result};
use std::sync::Arc;

// #[get("/ping")]
pub async fn create_user(
    // user_service: Arc<UserService<UserRepo>>,
    // req: Json<super::dto::CreateUserRequest>,
) -> Result<impl Responder> {
    let resp: DefaultResponse<String> = DefaultResponse {
        status: "0001".to_string(),
        message: "success".to_string(),
        data: None,
    };
    Ok(Json(resp))
}
