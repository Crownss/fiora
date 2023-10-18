mod common;
mod configuration;
mod data;
mod domain;
mod server;
use tokio;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // println!("Hello, world!");
    // let config = configuration::Configuration::new();
    // let theclient = data::infra::psql::get_connection()
    //     .await
    //     .as_ref()
    //     .expect("cannot get connection");
    data::infra::psql::check_connection()
        .await
        .as_ref()
        .expect("cannot ping the client");
}
