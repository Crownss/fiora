use actix_web::dev::ResourcePath;
use std::env;

use dotenv::dotenv;
use lazy_static::lazy_static;

lazy_static! {
    static ref CONFIGURATION: Configuration = Configuration::new();
}

pub fn get_configurations() -> &'static Configuration {
    &CONFIGURATION
}

pub struct PsqlConfiguration {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub db_name: String,
    pub time_out: i32,
    pub maxpool: u32,
}

pub struct ServerConfiguration {
    pub app_name: String,
    pub host: String,
    pub port: u16,
    pub client_timeout: u8,
    pub graceful: u8,
}

pub struct Configuration {
    pub psql: PsqlConfiguration,
    pub server: ServerConfiguration,
}

impl Default for PsqlConfiguration {
    fn default() -> Self {
        PsqlConfiguration {
            host: env::var("psql.host").expect("psql.host not found"),
            port: env::var("psql.port")
                .expect("psql.port not found")
                .parse()
                .unwrap(),
            username: env::var("psql.username").expect("psql.username not found"),
            password: env::var("psql.password").expect("psql.password not found"),
            db_name: env::var("psql.name").expect("psql.dbname not found"),
            time_out: env::var("psql.timeout")
                .expect("psql.timeout not found")
                .parse()
                .unwrap(),
            maxpool: env::var("psql.maxpool")
                .expect("psql.maxpool not found")
                .parse()
                .unwrap(),
        }
    }
}

impl Default for ServerConfiguration {
    fn default() -> Self {
        ServerConfiguration {
            app_name: env::var("appName").expect("appName not found"),
            host: env::var("HOST").expect("HOST not found").parse().unwrap(),
            port: env::var("PORT").expect("PORT not found").parse().unwrap(),
            graceful: env::var("graceful")
                .expect("graceful not found")
                .parse()
                .unwrap(),
            client_timeout: env::var("client.timeout")
                .expect("client.timeout not found")
                .parse()
                .unwrap(),
        }
    }
}

impl PsqlConfiguration {
    pub fn new() -> Self {
        PsqlConfiguration::default()
    }
}

impl ServerConfiguration {
    pub fn new() -> Self {
        ServerConfiguration::default()
    }
}

impl Default for Configuration {
    fn default() -> Configuration {
        Configuration {
            psql: PsqlConfiguration::new(),
            server: ServerConfiguration::new(),
        }
    }
}

impl Configuration {
    pub fn new() -> Self {
        dotenv()
            .ok()
            .expect("Unable to find .env file. Create one based on the env.example");
        Configuration::default()
    }
}
