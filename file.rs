use std::io::File;
use std::io::{Open, ReadWrite};

fn main() {
    // won't work if foo.txt doesn't exist!
    let path = Path::new("foo.txt");
    let mut stream = File::open_mode(&path, Open, ReadWrite);
    loop {
        let byte = stream.read_byte();
        match byte {
            Some(b) => println(b.to_str()),
            // EOF
            None => { println("EOF, all done"); break }
        }
    }
    stream.write(bytes!("foo!\n"));
}
