use crate::{common::errors::Res, configuration::get_configurations};
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    Pool, Postgres,
};
use std::{sync::Arc, time::Duration};
use tracing::info;

pub type TheClient = Arc<Pool<Postgres>>;

lazy_static! {
    static ref THECLIENT: AsyncOnce<Res<Pool<Postgres>>> =
        AsyncOnce::new(async { new_connection().await });
}

pub async fn new_connection() -> Res<Pool<Postgres>> {
    let config = get_configurations();
    let option = PgConnectOptions::new()
        .host(&config.psql.host)
        .port(config.psql.port)
        .database(&config.psql.db_name)
        .application_name(&config.server.app_name)
        .username(&config.psql.username)
        .password(&config.psql.password);

    let conn_manager = PgPoolOptions::new()
        .max_connections(config.psql.maxpool as u32)
        .max_lifetime(Duration::new(config.psql.time_out as u64, 0))
        .connect_with(option)
        .await?;
    Ok(conn_manager)
}

pub async fn get_connection() -> Res<Pool<Postgres>> {
    THECLIENT.get().await.clone()
}

pub async fn check_connection() -> Res<()> {
    info!("Checking on database connection...");
    let conn = get_connection().await?;
    let test: (String,) = sqlx::query_as("select $1::TEXT")
        .bind("pong")
        .fetch_one(&conn)
        .await?;
    info!(
        "Successfully connected to database and got response \"{}\"",
        test.0
    );
    Ok(())
}
