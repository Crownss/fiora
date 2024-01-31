use crate::configuration::get_configurations;
use crate::data::infra::psql::{check_connection, get_connection};
use crate::data::repo::user::repository::UserRepo;
use crate::data::repo::user::UserDataStore;
use crate::interactor::user::intr::UserService;
use crate::protocol::http::server::{Server, Services};
use std::sync::Arc;

pub async fn start() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    let config = get_configurations();
    check_connection().await.unwrap();
    let theclient = get_connection().await.unwrap();
    //user//
    let user_data_store = UserDataStore::new(theclient.clone());
    let user_repo = UserRepo::new(user_data_store);
    let user_service = Arc::new(UserService::new(user_repo));
    //user//

    let server = Server::new(
        config.server.graceful,
        config.server.client_timeout,
        config.server.host.clone(),
        config.server.port,
        Services { user_service },
    );
    server.run().await;
    // check_connection().await.unwrap();
    Ok(())
}
