use tokio::net::TcpListener

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("Accepted connection from {:?}", socket);
    }
}


