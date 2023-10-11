use tokio::net::TcpListener;
use bytes::BytesMut;

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("Accepted connection from {:?}", socket);
        let mut buf = BytesMut::with_capacity(1024);
        let _ = socket.try_read_buf(&mut buf);
        let attrs = buffer_to_array(&mut buf);
        println!("buffer {:?}", attrs);
    }
}


