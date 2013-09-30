use std::rt::io::*;
use std::rt::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::println;

fn main() {
    // try the first ip
    let ips = net::get_host_addresses("localhost").unwrap();
    debug2!("Saw {:?} for IPs", ips);
    let ip = ips.iter().filter(|x| match x { & &Ipv4Addr(*) => true, _ => false }).next().unwrap();
    debug2!("Picked {:?}", ip);

    let mut socket = net::tcp::TcpStream::connect(SocketAddr { ip: *ip, port: 13699 });
    socket.write(bytes!("hello, world"));
    let bytes = socket.read_bytes(12);
    println(std::str::from_utf8(bytes));
}
