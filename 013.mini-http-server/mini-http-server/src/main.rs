use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8088").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {

            handler::handle_request(socket).await;

        });
    }
}
