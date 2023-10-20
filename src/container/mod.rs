use crate::configuration::Configuration;
use crate::data::infra::psql::{get_connection, check_connection};
use crate::data::repo::user::repository::UserRepo;
use crate::data::repo::user::UserDataStore;
use crate::domain::user::user_service::UserService;
use crate::server::http::server::{Server, Services};
use std::ops::Deref;
use std::sync::Arc;
use tokio_postgres::GenericClient;

pub async fn start() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    let config = Configuration::new();
    let theclient = get_connection().await.unwrap();
    //user//
    let user_data_store = UserDataStore::new(theclient.clone());
    let user_repo = UserRepo::new(user_data_store);
    let user_service = Arc::new(UserService::new(user_repo));
    //user//

    let server = Server::new(
        config.server.graceful,
        config.server.client_timeout,
        config.server.port,
        Services { user_service },
    );
    server.run().await;
    // check_connection().await.unwrap();
    Ok(())
}
