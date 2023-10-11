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
        let command = Command::get_command(&attrs[0]);
        println!("buffer {:?}", attrs);
        process_query(command, attrs, &mut socket, &mut db).await?;
        
    }
}

async fn process_query(
    command: Command,
    attrs: Vec<String>,
    socket: &mut TcpStream,
    db: &mut Db,
) -> std::io::Result<()> {
    match command {
        Command::Get => {
            Ok(())
        }
        Command::Set => {
            let resp = db.write(&attrs);

            match resp {
                Ok(result) => {
                    println!("set result: {}", result);
                    socket.write_all(&result.as_bytes()).await?;
                }
                Err(_err) => {
                    socket.write_all(b"").await?;
                }
            }

            Ok(())
        }
        Command::Invalid => Ok(()),
    

let db = Db::new()
loop {
    let (socket, _) = listener.accept().await?;
}

