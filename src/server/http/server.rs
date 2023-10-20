use std::sync::Arc;
use std::time::Duration;

use actix_web::web::{get, scope, Data};
use actix_web::{middleware::Logger, App, HttpServer};

use crate::data::repo::user::repository::UserRepo;
use crate::domain::user::user_service::UserService;

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
    pub fn new(graceful: u8, client_timeout: u8, port: u16, service: Services) -> Self {
        Self {
            graceful,
            client_timeout,
            port,
            service,
        }
    }

    pub async fn run(&self) {
        let uservice = Data::new(self.service.user_service.clone());
        HttpServer::new(move|| {
            App::new()
        .wrap(Logger::new(
            "\n tz: %t\n got IP: %a\n with sec: %T\n in: %r\n and return: %s\n with headers: %{User-Agent}i",
        ))
        .service(
            scope("/v1/api")
                .app_data(uservice.clone())
                .route("/ping", get().to(user_handler::create_user))
                .route("/pong", get().to(user_handler::create_user)),
        )})
        .bind(format!("0.0.0.0:{}", self.port))
        .unwrap()
        .shutdown_timeout(self.graceful as u64)
        .client_request_timeout(Duration::new(self.client_timeout as u64, 0))
        .run()
        .await
        .unwrap();
    }
}
