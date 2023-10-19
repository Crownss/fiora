use crate::data::repo::user::repository::UserRepo;
use crate::domain::user::user_service::UserService;
use actix_web::web::get;
use actix_web::{App, HttpServer};
use std::sync::Arc;

use super::user::user_handler;

pub struct Services {
    pub user_service: Arc<UserService<UserRepo>>,
}

pub struct Server {
    pub port: u16,
    pub service: Services,
}

impl Server {
    pub fn new(port: u16, service: Services) -> Self {
        Self { port, service }
    }

    pub async fn run(&self) {
        HttpServer::new(|| App::new().route("/ping", get().to(user_handler::create_user)))
            .bind(format!("0.0.0.0:{}", self.port))
            .unwrap()
            .run()
            .await
            .unwrap();
    }
}
