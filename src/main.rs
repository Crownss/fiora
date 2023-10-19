use tokio;

mod common;
mod configuration;
mod data;
mod domain;
mod server;
mod container;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    container::start().await
}
