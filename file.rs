use std::rt::io::*;
use std::io::println;

fn main() {
    let path = Path("foo.txt");
    let mut stream = file::open(&path, OpenOrCreate, ReadWrite);
    loop {
        let byte = stream.read_byte();
        match byte {
            Some(b) => println(b.to_str()),
            // EOF
            None => { println("EOF, all done"); break }
        }
    }
}
