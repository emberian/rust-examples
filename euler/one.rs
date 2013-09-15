/// Find the sum of all the multiples of 3 or 5 below 1000.

use std::iter::AdditiveIterator;

struct MultipleIterator {
    priv current: uint
}

impl MultipleIterator {
    fn new() -> MultipleIterator {
        MultipleIterator { current: 0 }
    }
}

impl Iterator<uint> for MultipleIterator {
    fn next(&mut self) -> Option<uint> {
        let is_multiple = |x: uint| (x % 3 == 0) | (x % 5 == 0);

        while !is_multiple(self.current) {
            self.current += 1
        }
        self.current += 1;

        Some(self.current - 1)
    }
}

fn main() {
    let it = MultipleIterator::new();
    println!("{}", it.take_while(|&x| x < 1000).sum());
}
