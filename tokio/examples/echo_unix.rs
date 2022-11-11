//! A "hello world" echo server with Tokio
//!
//! This server will create a TCP listener, accept connections in a loop, and
//! write back everything that's read off of each TCP connection.
//!
//! Because the Tokio runtime uses a thread pool, each TCP connection is
//! processed concurrently with all other TCP connections across multiple
//! threads.
//!
//! To see this server in action, you can run this in one terminal:
//!
//!     cargo run --example echo
//!
//! and in another terminal you can run:
//!
//!     cargo run --example connect 127.0.0.1:8080
//!
//! Each line you type in to the `connect` terminal should be echo'd back to
//! you! If you open up multiple terminals running the `connect` example you
//! should be able to see them all make progress simultaneously.

#![warn(rust_2018_idioms)]

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::runtime;

use std::env;

//#[tokio::main]
//async fn main() -> Result<(), Box<dyn Error>> {
fn main() {
    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(4)
        .build()
        .unwrap();

    tokio::spawn(async {
        
    });

    rt.block_on(async {
        //let addr = env::args()
        //    .nth(1)
        //    .unwrap_or_else(|| "127.0.0.1:8080".to_string());

        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        //println!("Listening on: {}", addr);

        loop {
            // Asynchronously wait for an inbound socket.
            let (mut socket, _) = listener.accept().await.unwrap();
            println!("accept: {:?}", socket);

            tokio::spawn(async move {

                //let (mut reader, mut writer) = socket.into_split();
                //tokio::io::copy(&mut reader, &mut writer).await.unwrap();
                // In a loop, read data from the socket and write the data back.
                let mut buf = vec![0; 1024];
                loop {
                    let n = socket.read(&mut buf).await.unwrap();

                    if n == 0 {
                        return;
                    }

                    socket.write_all(&buf[0..n]).await.unwrap();
                }
            });
        }
    });
    println!("main exit");
}
