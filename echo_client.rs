use std::rt::io::*;
use std::rt::io::net::ip::SocketAddr;
use std::io::println;

fn main() {
    // try the first ip
    let ip = net::get_host_addresses("echo.octayn.net").unwrap()[0];
    let mut socket = net::tcp::TcpStream::connect(SocketAddr { ip: ip, port: 13699 });
    socket.write(bytes!("hello, world"));
    let bytes = socket.read_to_end();
    println(std::str::from_utf8(bytes));
}
