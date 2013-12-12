use std::io::net;
use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::net::tcp::TcpStream;

fn main() {
    // try the first ip
    let ips = net::get_host_addresses("localhost").unwrap();
    debug!("Saw {:?} for IPs", ips);
    let ip = ips.iter().filter(|x| match x { & &Ipv4Addr(..) => true, _ => false }).next().unwrap();
    debug!("Picked {}", ip.to_str());

    let mut socket = TcpStream::connect(SocketAddr { ip: *ip, port: 13699 });
    socket.write(bytes!("hello, world"));
    let bytes = socket.read_bytes(12);
    println(std::str::from_utf8(bytes));
}
