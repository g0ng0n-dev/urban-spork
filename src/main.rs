use std::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener= TcpListener::bind("localhost:8080").await.unwrap();
    let (mut socket,addr ) = listener.accept().await.unwrap();

    let (reader, mut write) = socket.split();
    // The Bufreader of tokyo wraps any kind of reader and it maintains its own buffer
    // and allows you to do some higher level read operations, for example read an entire line of text
    let mut reader = BufReader::new(&reader);

    let mut line = String::new();
    loop {

        let bytes_read = reader.read_line(&mut line).await.unwrap();
        if bytes_read == 0{
            break;
        }
        write.write_all(line.as_bytes()).await.unwrap();

        //we need to clear out the input buffer
        line.clear()
    }

}
