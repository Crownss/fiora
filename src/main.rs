use tokio;

mod common;
mod configuration;
mod container;
mod data;
mod interactor;
mod protocol;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    container::start().await
}
