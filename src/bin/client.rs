use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let mut buf = BytesMut::with_capacity(1024);
    let _length = stream.read_buf(&mut buf).await?;
    match std::str::from_utf8(&mut buf) {
        Ok(resp) => {
            if resp == "r Ok" {
                println!("key updated");
            } else if resp == "Ok" {
                println!("key set");
            }
        }
        Err(err: Utf8Error) => {
            println!("error: {}", err);
        }
    }
    
}