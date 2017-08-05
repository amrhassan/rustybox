
use std::collections::HashMap;

pub fn factorize(n: u64, primes: &mut CachedPrimes) -> Vec<u64> {

    let mut factors = Vec::new();

    let mut buff = n;
    while buff > 1 {
        let factor = primes.iter().find(|p| buff % *p == 0).unwrap(); // Should never be None
        factors.push(factor);
        buff = buff / factor;
    };

    factors
}

/// Unique prime factors and their occurrence counts
pub fn factorize_unique(n: u64, primes: &mut CachedPrimes) -> HashMap<u64, u32> {
    let mut result: HashMap<u64, u32> = HashMap::new();
    for factor in factorize(n, primes) {
        let new_count = result.get(&factor).unwrap_or(&0) + 1;
        result.insert(factor, new_count);
    }
    result
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
        self.n = next_after(prime);
        Some(prime)
    }
}

fn next_after(n: u64) -> u64 {
    (n+1..).find(|&pc| is_prime(pc)).expect("Failed to find the next prime for some reason")
}

pub struct CachedPrimes { values: Vec<u64> }

impl CachedPrimes {
    pub fn with_capacity(c: usize) -> CachedPrimes {
        CachedPrimes { values: Vec::with_capacity(c) }
    }

    pub fn new() -> CachedPrimes {
        CachedPrimes { values: Vec::new() }
    }

    pub fn iter(&mut self) -> CachedPrimesIterator {
        CachedPrimesIterator { cached: self, next_index_to_serve: 0 }
    }
}

struct CachedPrimesIterator<'a> {
    cached: &'a mut CachedPrimes,
    next_index_to_serve: usize,
}

impl<'a> Iterator for CachedPrimesIterator<'a> {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        if self.next_index_to_serve == self.cached.values.len() {
            let next = next_after(*self.cached.values.last().unwrap_or(&1));
            self.cached.values.push(next);
            self.next_index_to_serve += 1;
            Some(next)
        } else {
            let next = *self.cached.values.get(self.next_index_to_serve).expect("Failed to find prime in cache");
            self.next_index_to_serve += 1;
            Some(next)
        }
    }
}

pub fn is_prime(n: u64) -> bool {
    let sqrt_n: u64 = (n as f64).sqrt() as u64;
    !(2..(sqrt_n+1)).any(|divisor| n % divisor == 0)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn cached_primes_are_correct() {
        let mut cached_primes = CachedPrimes::new();
        let cached_1: Vec<u64> = cached_primes.iter().take(10).collect();
        let cached_2: Vec<u64> = cached_primes.iter().take(10).collect();
        let dynamic: Vec<u64> = PrimesIterator::new().take(10).collect();

        assert_eq!(cached_1, dynamic);
        assert_eq!(cached_1, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
        assert_eq!(cached_1, cached_2);
    }
}
