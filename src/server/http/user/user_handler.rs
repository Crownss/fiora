use crate::common::errors::Res;
use crate::common::responses::DefaultResponse;
use crate::data::repo::user::repository::UserRepo;
use crate::domain::user::user_model::User;
use crate::domain::user::user_service::UserService;
use actix_web::web::{Data, Json};
use actix_web::{get, HttpResponse, Responder, Result};
use std::sync::Arc;
use tracing::{error, info};

// #[get("/ping")]
// struct UserHandler {
//     pub handler: Arc<UserService<UserRepo>>,
// }
// impl UserHandler {
//     pub fn new(&self) -> Self {
//         Self { handler: self.handler }
//     }
// }
pub async fn create_user(user_service: Data<Arc<UserService<UserRepo>>>) -> Result<impl Responder> {
    let list_user = user_service.list_user().await;
    let mut  resp: DefaultResponse<Vec<User>> = DefaultResponse{
        status: "0001".to_string(),
        message: "success".to_string(),
        data: None,
    };
    match list_user {
        Ok(res) => resp.data = Some(res),
        Err(err) => resp.data = None,
    };
    // let resp: DefaultResponse<String> = DefaultResponse {
    //     status: "0001".to_string(),
    //     message: "success".to_string(),
    //     data: None,
    // };
    Ok(Json(resp))
}
