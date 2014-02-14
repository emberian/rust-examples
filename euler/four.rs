struct DivisibleIterator {
    current: uint,
    divisors: ~[uint],
}

impl DivisibleIterator {
    fn new(divisors: ~[uint]) -> DivisibleIterator {
        DivisibleIterator {
            current: 0,
            divisors: divisors
        }
    }
}

impl Iterator<uint> for DivisibleIterator {
    /// Yield the next number divisible by all of the divisors
    fn next(&mut self) -> Option<uint> {
        self.current += 1;

        while !self.divisors.iter().all(|&x| self.current % x == 0) {
            self.current += 1;
        }
        Some(self.current)
    }
}

fn main() {
    let mut it = DivisibleIterator::new(range(1u, 20).to_owned_vec());
    println!("{}", it.next().unwrap());
}
