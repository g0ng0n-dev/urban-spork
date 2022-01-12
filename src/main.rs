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
            // what the select Macro does is allows you to run multiple asyncrhnous things concurrently
            // at the same time and act on the first one that comes back with a result
            tokio::select!{
                // identifier -> feature -> block of code
                result = reader.read_line(&mut line) => {
                     if result.unwrap() == 0{
                        break;
                    }

                    tx.send(line.clone()).unwrap();
                    line.clear();
                }

                //rx.recv returns a future, we unwrap the future into the msg var
                // the select macro is going to basically going to do awieght on the future
                // implicitly as a part of the compiler machinery that it generates.
                result = rx.recv() => {
                    let msg = result.unwrap();
                    write.write_all(msg.as_bytes()).await.unwrap();
                }
            }


            //we need to clear out the input buffer
        }
    });


}
