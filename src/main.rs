use std::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

// a Turbofish ::<Type> ... is an operator that is pretty unique to Rust.
// it is essentially a way to hing to the compiler, what kind of
// a generic type you expect to be returned from a function.
#[tokio::main]
async fn main() {
    let listener= TcpListener::bind("localhost:8080").await.unwrap();

    // A broadcast channel is a special type of channel which allows multiple producers
    // and multiple consumers to all send and receiev on a single channel.
    // basically we can have a sender and a receiever on every single Task for handling our clients.
    // and each client task will send lines that it reads from its own client when they're ready
    // and then, it will also receieve lines read in from other clients as soon as they have written
    // their lines
    let (tx, _rx) = broadcast::channel(10);


    let tx = tx.clone();
    let mut rx = tx.subscribe();

    // Task is essentially a unit of work in the async world, lightweight thread
    // here we create a new task to handle multiple clients call
    // Rust actually has the concept of an async block, which is basically wrapping up
    // one little piece of code into its own future, its own unit of async work
    // every time you call tokyo spawn the thing that youo pass into it has to be static
    // so it has to have no stack references in it.
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
            // Select is very useful when you have things that need to operate on the same shared state,
            // when you have a finite number of things.
            // in the next example we have two async task, write msg and read msg
            tokio::select!{
                // identifier -> feature -> block of code
                // when using select the futures block when entering in the block of code
                // also you can easily share references between all the different branches of select
                // it just makes memory management alot easier
                result = reader.read_line(&mut line) => {
                     if result.unwrap() == 0{
                        break;
                    }

                    tx.send((line.clone(), addr)).unwrap();
                    line.clear();
                }

                //rx.recv returns a future, we unwrap the future into the msg var
                // the select macro is going to basically going to do awieght on the future
                // implicitly as a part of the compiler machinery that it generates.
                result = rx.recv() => {
                    let (msg, other_addr) = result.unwrap();
                    if addr != other_addr{
                        write.write_all(msg.as_bytes()).await.unwrap();
                    }
                }
            }


            //we need to clear out the input buffer
        }
    });


}
