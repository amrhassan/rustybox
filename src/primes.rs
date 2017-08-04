

pub fn factorize(n: u64) -> Vec<u64> {

    let mut factors = Vec::new();

    let mut buff = n;
    while buff > 1 {
        let factor = PrimesIterator::new().find(|p| buff % *p == 0).unwrap(); // Should never be None
        factors.push(factor);
        buff = buff / factor;
    };

    factors
}

pub struct PrimesIterator {n: u64}

impl PrimesIterator {
    pub fn new() -> PrimesIterator {
        PrimesIterator{n: 2}
    }
}

impl Iterator for PrimesIterator {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let prime = self.n;
        let next_prime = ((prime+1)..).find(|&n| is_prime(n)).unwrap();
        self.n = next_prime;
        Some(prime)
    }
}


pub fn is_prime(n: u64) -> bool {
    let sqrt_n: u64 = (n as f64).sqrt() as u64;
    !(2..(sqrt_n+1)).any(|divisor| n % divisor == 0)
}
