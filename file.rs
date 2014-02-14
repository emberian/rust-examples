use std::io::File;
use std::io::{Open, ReadWrite};

// won't work if foo.txt doesn't exist!

fn main() {
    let path = Path::new("foo.txt");
    // instead of unwraps, one could handle error
    let mut stream = File::open(&path).ok().expect("could not open file :(");
    loop {
        let byte = stream.read_byte();
        match byte {
            Ok(b) => println!("{}", b),
            // EOF
            Err(d) => { println!("Error (EOF?): {}", d); break }
        }
    }
    stream.write(bytes!("foo!\n"));
}
