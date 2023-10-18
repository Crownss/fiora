use crate::common::errors::{CustomError, Res};
use crate::configuration::Configuration;
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use tokio_postgres::tls::NoTlsStream;
use tokio_postgres::{Client, Connection, NoTls, Socket};
use tracing::{error, info};

lazy_static! {}

pub type TheClient = Client;
// pub type TheConnection = Connection<Socket, NoTlsStream>;

// pub struct PsqlConn {
//     pub the_client: Client,
//     pub the_connection: Connection<Socket, NoTlsStream>,
// }

lazy_static! {
    static ref THECLIENT: AsyncOnce<Res<Client>> = AsyncOnce::new(async { new_connection().await });
}

pub async fn new_connection() -> Res<Client> {
    let config = Configuration::new();
    let (client, conn) = tokio_postgres::connect(
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
    tokio::spawn(async move {
        if let Err(e) = conn.await {
            error!("disconnected from postgres and got err: {}", e);
        }
    });
    // Ok(PsqlConn {
    //     the_client: client,
    //     the_connection: conn,
    // })
    Ok(client)
}

pub async fn get_connection<'a>() -> &'a Res<Client> {
    THECLIENT.get().await.clone()
}

pub async fn check_connection() -> Res<()> {
    info!("Checking on database connection...");
    let conn = get_connection().await;
    let test = conn.as_ref().unwrap().query_one("select $1::TEXT", &[&"pong"]).await;
    match test {
        Ok(row) => {
            let res: String = row.get(0);
            info!("connected! and got response: {}", res)
        }
        Err(err) => error!("{err}"),
    }
    // let client = get_connection().await.unwrap().the_client;
    // tokio::spawn(async move {
    //     if let Err(e) = conn.unwrap().the_connection.await {
    //         error!("disconnected from postgres and got err: {}", e);
    //     }
    // });
    Ok(())
}
