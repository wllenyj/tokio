extern crate tokio;

use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::{Future, Stream};
use tokio::io::{lines};

use std::io::{BufReader};
use std::net::SocketAddr;

fn process(socket: TcpStream) {
    let conn = lines(BufReader::new(socket))
        .for_each(|line| {
            println!("read: {}", line);
            Ok(())
        })
        .map(|a| {
            println!("{:?} exit.", a);
        })
        .map_err(|e| {
            eprintln!("connection err = {:?}", e);
        });

    tokio::spawn(conn);
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let addr: SocketAddr = ":6142".parse()?;
    let addr = "0.0.0.0:6142".parse()?;
    let listener = TcpListener::bind(&addr)?;

    println!("server running on {}", listener.local_addr()?);

    let server = listener.incoming()
        .for_each(move |socket| {
            println!("incoming : {}", socket.peer_addr()?);
            process(socket);
            Ok(())
        })
        .map_err(|err| {
            eprintln!("accept error = {:?}", err);
        });

    tokio::run(server);

    Ok(())
}
