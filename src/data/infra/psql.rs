use std::future::Future;
use std::sync::Arc;
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use tokio_postgres::{Client, GenericClient, NoTls, Config, Connection};
use tracing::{error, info};

use crate::common::errors::{CustomError, Res};
use crate::configuration::Configuration;

pub type TheClient = Client;

lazy_static! {
    static ref THECLIENT: AsyncOnce<Res<Client>> = AsyncOnce::new(async { new_connection().await });
}

pub async fn new_connection() -> Res<Client> {
    let config = Configuration::new();
    let configss = Config::new()
        .user("nama_pengguna")
        .password("kata_sandi")
        .host("alamat_host")
        .port(5432)
        .dbname("nama_database")
        .connect(NoTls);
    let (_, conn) = tokio_postgres::connect(
        format!(
            "host={} port={} user={} password={} dbname={} connect_timeout={}",
            config.psql.host,
            config.psql.port,
            config.psql.username,
            config.psql.password,
            config.psql.db_name,
            config.psql.time_out
        )
        .as_str(),
        NoTls,
    )
    .await?;
    // tokio::spawn(async move {
    //     if let Err(e) = conn.await {
    //         error!("disconnected from postgres and got err: {}", e);
    //     }
    // });
    let pool = conn.poll();
    Ok(client)
}

pub async fn get_connection<'a>() -> Arc<Res<&'a Client>>{
    Arc::new(THECLIENT.get().await.clone())
}

pub async fn check_connection() -> Res<()> {
    info!("Checking on database connection...");
    let conn = get_connection().await;
    let test = conn
        .as_ref()
        .unwrap()
        .query_one("select $1::TEXT", &[&"pong"])
        .await?;
    match test {
        Ok(row) => {
            let res: String = row.get(0);
            info!("connected! and got response: {}", res)
        }
        Err(err) => error!("{err}"),
    }
    Ok(())
}
