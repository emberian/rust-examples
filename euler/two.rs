/// Find the sum of all the multiples of 3 or 5 below 1000.

use std::iter::AdditiveIterator;

struct FibIterator {
    priv current: uint,
    priv prev: uint,
}

impl FibIterator {
    fn new() -> FibIterator {
        FibIterator { current: 1, prev: 0 }
    }
}

impl Iterator<uint> for FibIterator {
    fn next(&mut self) -> Option<uint> {
        let (a, b) = (self.current, self.current + self.prev);
        self.current = b;
        self.prev = a;
        Some(b)
    }
}

fn main() {
    let it = FibIterator::new();
    println!("{}", it.filter(|&x| x % 2 == 0).take_while(|&x| x < 4_000_000).sum());
}
