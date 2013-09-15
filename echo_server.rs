use std::rt::io::*;
use std::rt::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::println;

fn main() {
    // try the first ip
    let socket = net::tcp::TcpListener::bind(SocketAddr { ip: Ipv4Addr(0, 0, 0, 0), port: 13699 });
    let mut acceptor = socket.listen().unwrap();
    loop {
        let mut stream = acceptor.accept();
        let bytes = stream.read_to_end();
        stream.write(bytes);
    }
}
