use tokio;

mod common;
mod configuration;
mod data;
mod interactor;
mod protocol;
mod container;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    container::start().await
}
