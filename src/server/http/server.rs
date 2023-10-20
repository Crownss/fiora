use crate::data::repo::user::repository::UserRepo;
use crate::domain::user::user_service::UserService;
use actix_web::web::get;
use actix_web::{App, HttpServer};
use std::sync::Arc;
use std::time::Duration;

use super::user::user_handler;

pub struct Services {
    pub user_service: Arc<UserService<UserRepo>>,
}

pub struct Server {
    pub graceful: u8,
    pub client_timeout: u8,
    pub port: u16,
    pub service: Services,
}

impl Server {
    pub fn new(graceful:u8, client_timeout:u8, port: u16, service: Services) -> Self {
        Self { graceful, client_timeout, port, service }
    }

    pub async fn run(&self) {
        HttpServer::new(|| App::new().route("/ping", get().to(user_handler::create_user)))
            .bind(format!("0.0.0.0:{}", self.port))
            .unwrap()
            .shutdown_timeout(self.graceful as u64)
            .client_request_timeout(Duration::new(self.client_timeout as u64, 0))
            .run()
            .await
            .unwrap();
    }
}
