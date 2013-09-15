/// Find the sum of all the multiples of 3 or 5 below 1000.

use std::iter::AdditiveIterator;

struct PrimeFactorIterator {
    priv number: uint,
    priv current: uint,
}

impl PrimeFactorIterator {
    fn new(n: uint) -> PrimeFactorIterator {
        PrimeFactorIterator {
            number: n,
            current: 1,
        }
    }
}

impl Iterator<uint> for PrimeFactorIterator {
    fn next(&mut self) -> Option<uint> {
        fn isprime(n: uint) -> bool {
            range(2, (n as float).sqrt().ceil() as uint).all(|x| n % x != 0)
        }

        while self.current < (self.number as float).sqrt().ceil() as uint  {
            self.current += 1;
            if (self.number % self.current == 0) & isprime(self.current) {
                return Some(self.current);
            }
        }
        None
    }
}

fn main() {
    let mut it = PrimeFactorIterator::new(600851475143);
    for factor in it {
        println!("{}", factor);
    }
}
