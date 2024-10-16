use std::sync::Arc;
use std::time::Duration;

use actix_web::web::{get, post, scope, Data};
use actix_web::HttpRequest;
use actix_web::{guard, App, HttpServer};

use crate::data::repo::user::repository::UserRepo;
use crate::interactor::auth::intr::AuthService;

use super::user::user_handler;

pub struct Services {
    pub user_service: Arc<AuthService<UserRepo>>,
}

pub struct Server {
    pub graceful: u8,
    pub client_timeout: u8,
    pub host: String,
    pub port: u16,
    pub service: Services,
}

impl Server {
    pub fn new(
        graceful: u8,
        client_timeout: u8,
        host: String,
        port: u16,
        service: Services,
    ) -> Self {
        Self {
            graceful,
            client_timeout,
            host,
            port,
            service,
        }
    }

    pub async fn run(&self) {
        let uservice = Data::new(self.service.user_service.clone());
        HttpServer::new(move || {
            App::new()
                .wrap(super::middleware::request::Logging)
                .service(
                    scope("/v1/api")
                        .guard(guard::Any(guard::Get()).or(guard::Post()))
                        .app_data(uservice.clone())
                        // .route("/users", get().to(user_handler::list_users))
                        .route("/users", post().to(user_handler::register))
                        // .route("/users/getby", get().to(user_handler::get_user_by))
                        .route(
                            "/ping",
                            get().to(|_req: HttpRequest| async { "PONG!".to_owned() }),
                        ),
                )
        })
        .bind(format!("{}:{}", self.host, self.port))
        .unwrap()
        .shutdown_timeout(self.graceful as u64)
        .client_request_timeout(Duration::new(self.client_timeout as u64, 0))
        // .workers(1)
        .run()
        .await
        .unwrap();
    }
}
