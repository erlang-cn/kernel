extern crate mio;
extern crate slab;
extern crate core;
extern crate kernel;

use std::net::SocketAddr;
use kernel::io::poll::*;
use kernel::io::tcp::*;
use kernel::io::server::*;

fn main() {
    println!("IO Server started");
    let addr = "127.0.0.1:8000".parse::<SocketAddr>()
        .ok().expect("Failed to parse host:port string");
    let sock = TcpListener::bind(&addr).ok().expect("Failed to bind address");
    let mut poll = Poll::new().expect("Failed to create Poll");
    let mut server = Server::new(sock);
    server.run(&mut poll).expect("Failed to run server");
}
