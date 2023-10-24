use std::sync::Arc;
use std::time::Duration;

use actix_web::web::{get, post, scope, Data};
use actix_web::{dev::Service as _, middleware::Logger, App, HttpServer};
use futures_util::FutureExt;
use tracing::info;

use crate::data::repo::user::repository::UserRepo;
use crate::interactor::user::user_service::UserService;

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
                .wrap(super::middleware::request::Logging)
        .wrap(Logger::new(
            "\n uri: %U\n got IP: %{r}a\n with sec: %T\n in: %r\n and return: %s\n with user-agent: %{User-Agent}i",
        ))
        .service(
            scope("/v1/api")
                .app_data(uservice.clone())
                .route("/users", get().to(user_handler::list_users))
                .route("/users", post().to(user_handler::create_users)),
        )})
        .bind(format!("0.0.0.0:{}", self.port))
        .unwrap()
        .shutdown_timeout(self.graceful as u64)
        .client_request_timeout(Duration::new(self.client_timeout as u64, 0))
        // .workers(1)
        .run()
        .await
        .unwrap();
    }
}
