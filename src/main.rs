use std::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;


#[tokio::main]
async fn main() {
    let listener= TcpListener::bind("localhost:8080").await.unwrap();

    // A broadcast channel is a special type of channel which allows multiple producers
    // and multiple consumers to all send and receiev on a single channel.
    // basically we can have a sender and a receiever on every single Task for handling our clients.
    // and each client task will send lines that it reads from its own client when they're ready
    // and then, it will also receieve lines read in from other clients as soon as they have written
    // their lines
    let (tx, _rx) = broadcast::channel::<String>(10);


    let tx = tx.clone();
    let mut rx = tx.subscribe();

    // Task is essentially a unit of work in the async world, lightweight thread
    // here we create a new task to handle multiple clients call
    // Rust actually has the concept of an async block, which is basically wrapping up
    // one little piece of code into its own future, its own unit of async work
    tokio::spawn(async move {
        let (mut socket,addr ) = listener.accept().await.unwrap();

        let (reader, mut write) = socket.split();
        // The Bufreader of tokyo wraps any kind of reader and it maintains its own buffer
        // and allows you to do some higher level read operations, for example read an entire line of text
        let mut reader = BufReader::new(reader);

        let mut line = String::new();
        loop {

            let bytes_read = reader.read_line(&mut line).await.unwrap();
            if bytes_read == 0{
                break;
            }
            tx.send(line.clone()).unwrap();

            let msg = rx.recv().await.unwrap();

            write.write_all(msg.as_bytes()).await.unwrap();

            //we need to clear out the input buffer
            line.clear();
        }
    });


}
