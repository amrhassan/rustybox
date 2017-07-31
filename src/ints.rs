
// 5

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


pub fn smallest_number_divisible_by_ns_from_1_to_n__fast(n: u64) -> u64 {
    least_common_multiple((1..n+1).collect())
}

/// http://mathforum.org/library/drmath/view/62527.html
pub fn least_common_multiple(ns: Vec<u64>) -> u64 {

    let mut factors_and_exponents: HashMap<u64, u32> = HashMap::new();

    for n in &ns {
        let mut n_factor_count = HashMap::new();
        for factor in primes::factorize(*n) {
            let new_count = n_factor_count.get(&factor).unwrap_or(&0) + 1;
            n_factor_count.insert(factor, new_count);
        }
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
