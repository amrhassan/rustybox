
use primes;
use std::collections::HashMap;

fn is_divisible_by(lhs: u64, rhs: u64) -> bool {
    lhs % rhs == 0
}

fn is_divisible_by_range<F : Iterator<Item=u64>>(lhs: u64, rhs: F)  -> bool {
    rhs.fold(true, |acc, d| acc && is_divisible_by(lhs, d))
}

fn is_divisible_by_ns(lhs: u64, rhs: &Vec<u64>)  -> bool {
    rhs.iter().fold(true, |acc, d| acc && is_divisible_by(lhs, *d))
}

/// Returns the minimal set of numbers which are multipliers of all the numbers in the range [1..n]
fn biggest_multiples_of_numbers_in_range_1_to_n(n: u64) -> Vec<u64> {
    (1..n+1).filter(|x| {
        (1..n).find(|k| x % k == 0 && (x+k) > n).is_some() // Checks if is biggest multiplier available
    }).collect()
}

/// Brute force
pub fn smallest_number_divisible_by_ns_from_1_to_n(n: u64) -> u64 {
    let ns = biggest_multiples_of_numbers_in_range_1_to_n(n);
    (1..).find(|x| is_divisible_by_ns(*x, &ns)).unwrap()
}

pub fn smallest_number_divisible_by_ns_from_1_to_n__fast(n: u64, ps: &mut primes::CachedPrimes) -> u64 {
    least_common_multiple((1..n+1).collect(), ps)
}

/// http://mathforum.org/library/drmath/view/62527.html
pub fn least_common_multiple(ns: Vec<u64>, ps: &mut primes::CachedPrimes) -> u64 {

    let mut factors_and_exponents: HashMap<u64, u32> = HashMap::new();

    for n in ns {
        let n_factor_count = primes::factorize_unique(n, ps);
        for (factor, count) in n_factor_count.iter() {
            if count > factors_and_exponents.get(factor).unwrap_or(&0) {
                factors_and_exponents.insert(*factor, *count);
            }
        }
    }

    let mut result = 1;
    for (factor, exponent) in factors_and_exponents {
        result *= factor.pow(exponent);
    }

    result
}

pub struct PythagoreanTriplets {
    a: u32,
    b: u32,
    c: u32
}

impl PythagoreanTriplets {
    pub fn new() -> PythagoreanTriplets {
        PythagoreanTriplets { a: 1, b: 1, c: 1}
    }
}

impl Iterator for PythagoreanTriplets {

    type Item = (u32, u32, u32);

    fn next (&mut self) -> Option<(u32, u32, u32)> {
        'outermost: for c in self.c.. {
            for b in self.b..c {
                for a in (self.a+1)..b {
                    if a.pow(2) + b.pow(2) == c.pow(2) {
                        self.a = a;
                        self.b = b;
                        self.c = c;
                        break 'outermost;
                    }
                }
                self.a = 1;
            }
            self.b = 1;
        }
        Some((self.a, self.b, self.c))
    }
}

pub struct Triangular {
    next: u64,
    next_index: usize,
}

impl Iterator for Triangular {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let next = self.next;
        self.next = self.next + self.next_index as u64;
        self.next_index += 1;
        Some(next)
    }
}

impl Triangular {
    pub fn new() -> Triangular {
        Triangular { next: 1, next_index: 2}
    }
}

pub struct Divisors {
    n: u64,
    next_divisor: u64,
}

impl Iterator for Divisors {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let factor = self.next_divisor;
        if factor > 0 {
            self.next_divisor-= 1;
        }
        while (self.next_divisor > 0) && (self.n % self.next_divisor!= 0) {
            self.next_divisor -= 1;
        }
        if factor == 0 {
            None
        } else {
            Some(factor)
        }
    }
}

pub fn divisors(n: u64) -> Divisors {
    Divisors { n: n, next_divisor: n }
}

pub fn divisor_count(n: u64, ps: &mut primes::CachedPrimes) -> u32 {
    primes::factorize_unique(n, ps)
        .values()
        .map(|v| v + 1)
        .product()
}

pub fn factorial(n: u64) -> u64 {
    if n == 0 || n == 1 {
        1
    } else {
        n * factorial(n-1)
    }
}

pub fn combination(n: u64, k: u64) -> u64 {
    let num: u64 = (k+2..n).product();
    let denom: u64 = (2..k).product();
    num / denom
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn factorize_is_correct() {
        let one: Vec<u64> = divisors(1).collect();
        let three: Vec<u64> = divisors(3).collect();
        let twenty_eight: Vec<u64> = divisors(28).collect();

        assert_eq!(one, vec![1]);
        assert_eq!(three, vec![3, 1]);
        assert_eq!(twenty_eight, vec![28, 14, 7, 4, 2, 1]);
    }

    #[test]
    fn divisor_count_is_correct() {
        let mut ps = primes::CachedPrimes::new();

        assert_eq!(divisor_count(1, &mut ps), 1);
        assert_eq!(divisor_count(3, &mut ps), 2);
        assert_eq!(divisor_count(15, &mut ps), 4);
        assert_eq!(divisor_count(21, &mut ps), 4);
        assert_eq!(divisor_count(28, &mut ps), 6);
    }
}
