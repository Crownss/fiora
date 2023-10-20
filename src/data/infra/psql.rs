use std::time::Duration;
use async_once::AsyncOnce;
use bb8::{ManageConnection, Pool};
use bb8_postgres::{PostgresConnectionManager, tokio_postgres::{NoTls, Config}};
use lazy_static::lazy_static;
use tracing::{error, info};

use crate::common::errors::Res;
use crate::configuration::Configuration;

pub type TheClient = Pool<PostgresConnectionManager<NoTls>>;

lazy_static! {
    static ref THECLIENT: AsyncOnce<Res<Pool<PostgresConnectionManager<NoTls>>>> = AsyncOnce::new(async { new_connection().await });
}

pub async fn new_connection() -> Res<Pool<PostgresConnectionManager<NoTls>>> {
    let config = Configuration::new();
    let configss = Config::new()
        .host(config.psql.host.trim())
        .port(config.psql.port.parse().unwrap())
        .user(config.psql.username.trim())
        .password(config.psql.password.trim())
        .dbname(config.psql.db_name.trim())
        .application_name(config.server.app_name.trim())
        .connect_timeout(Duration::new(config.psql.time_out as u64, 0))
        .to_owned();
    // let theuri = format!("postgres://{}:{}@{}:{}/{}?connect_timeout={}&application_name={}",
    //                      config.psql.username.trim(),
    //                      config.psql.password.trim(),
    //                      config.psql.host.trim(),
    //                      config.psql.port.parse::<u32>().unwrap(),
    //                      config.psql.password.trim(),
    //                      config.psql.time_out,
    //                      config.server.app_name.trim(),
    // ).trim();
    let conn_manager = bb8_postgres::PostgresConnectionManager::new(configss, NoTls);
    let the_pool = bb8::Pool::builder().max_size(10).build(conn_manager.clone()).await.expect("cannot create connection for pool postgres");
    tokio::spawn(async move{
        let _ = conn_manager.connect();
    });
    Ok(the_pool)
}

pub async fn get_connection() -> Res<Pool<PostgresConnectionManager<NoTls>>> {
    THECLIENT.get().await.clone()
}

pub async fn check_connection() -> Res<()> {
    info!("Checking on database connection...");
    let conn = get_connection().await?;
    let test = conn.get().await.unwrap();
    let testconn = test.query_one("select $1::TEXT", &[&"pong"]).await;
    match testconn {
        Ok(row) => {
            let res: String = row.get(0);
            info!("connected! and got response: {}", res)
        }
        Err(err) => error!("{err}"),
    }
    Ok(())
}
