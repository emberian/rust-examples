use std::io::{Listener, Acceptor};
use std::io::net::tcp::TcpListener;
use std::io::net::ip::{SocketAddr, Ipv4Addr};

fn main() {
    // try the first ip
    let socket = TcpListener::bind(SocketAddr { ip: Ipv4Addr(0, 0, 0, 0), port: 13699 }).unwrap();
    let mut acceptor = socket.listen().unwrap();

    loop {
        let mut stream = acceptor.accept();
        println!("Saw connection!");
        let mut bytes = [0, ..1];
        loop {
            match stream.read(bytes) {
                Some(_) => stream.write(bytes),
                None    => break
            }
        }
    }
}
