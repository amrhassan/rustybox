
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;
use std::ops::Div;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use std::iter::repeat;
use std::cmp::max;
use std::iter::Sum;
use std::clone::Clone;
use std::cmp::Ordering;

/// Arbitrarily-sized unsigned BCD integer, MSB first.
/// Not a very efficient implementation as each decimal digit is stored in a whole byte.
pub struct BigInt { digits: Vec<u8> }

impl BigInt {

    pub fn zero() -> BigInt {
        BigInt { digits: Vec::new() }
    }

    pub fn of(n: u64) -> BigInt {
        BigInt::parse(&format!("{}", n)).expect("This should never happen")
    }

    pub fn parse(ns: &str) -> Option<BigInt> {
        let mut n = BigInt { digits: Vec::new() };
        let mut all_digits = true;
        for c in ns.chars() {
            match c.to_digit(10) {
                Some(d) => n.digits.push(d as u8),
                None => all_digits = false
            }
        }
        if all_digits {
            Some(n)
        } else { None }
    }

    /// Decimal left-shift. Effectively multiplies by 10^n.
    fn decimal_lsh(self, n: usize) -> BigInt {
        let mut digits = self.digits;
        digits.extend(repeat(0).take(n));
        BigInt { digits: digits }
    }

    /// Makes sure each digit contains no more than a single decimal digit
    fn spread_decimals_out(self) -> BigInt {

        let mut out = self.digits;

        let mut carry = 0;
        for ix in (0..out.len()).rev() {
            out[ix] = out[ix] + carry;
            carry = 0;
            if out[ix] > 9 {
                let original_value = out[ix];
                let final_value = original_value % 10;
                out[ix] = final_value;
                carry = (original_value - final_value) / 10;
            }
        }
        if carry > 0 {
            out.insert(0, carry);
        }

        BigInt { digits: out }
    }

    fn zero_padded_to(self, n: usize) -> BigInt {
        BigInt { digits: repeat(0u8).take(n - self.digits.len()).chain(self.digits.into_iter()).collect() }
    }

    fn sanity_check(&self) {
        for digit in &self.digits {
            if *digit > 9 {
                panic!("BigInt is not sane")
            }
        }
    }
}

impl Add for BigInt {

    type Output = BigInt;

    fn add(self, other: BigInt) -> BigInt {
        let max_length = max(self.digits.len(), other.digits.len());

        let lhs = self.zero_padded_to(max_length).digits.into_iter();
        let rhs = other.zero_padded_to(max_length).digits.into_iter();

        let out: Vec<u8> = lhs.zip(rhs).map(|(x, y)| x + y).collect();

        BigInt { digits: out }.spread_decimals_out()
    }
}

impl Sub for BigInt {

    type Output = BigInt;

    fn sub(self, other: BigInt) -> BigInt {

        if self < other {
            panic!("Trying to subtract a smaller number from a bigger number");
        }

        let max_length = max(self.digits.len(), other.digits.len());
        let mut lhs = self.zero_padded_to(max_length);
        let rhs = other.zero_padded_to(max_length);

        let mut reversed_out = Vec::new();
        for ix in (0..max_length).rev() {
            while lhs.digits[ix] < rhs.digits[ix] {
                let c = (1..).find(|&n| lhs.digits[ix-n] > 0).expect("Are you sure you're not subtracting a number from a smaller number?");
                lhs.digits[ix-c] -= 1;
                lhs.digits[ix-c+1] += 10;
            }
            reversed_out.push(lhs.digits[ix] - rhs.digits[ix]);
        };

        BigInt { digits: reversed_out.into_iter().rev().collect() }.spread_decimals_out()
    }
}

impl Mul for BigInt {

    type Output = BigInt;

    fn mul(self, rhs: BigInt) -> BigInt {

        let digit_multiples =
            rhs.digits
            .iter()
            .rev()
            .enumerate()
            .map(|(ix, &rhs_d)| {
                let digits = self.digits.iter().map(|lhs_d| lhs_d * rhs_d).collect();
                BigInt { digits }.spread_decimals_out().decimal_lsh(ix)
            });

        digit_multiples.sum()
    }
}

impl Div for BigInt {

    type Output = BigInt;

    fn div(self, rhs: BigInt) -> BigInt {

        let mut out = BigInt::zero();
        let mut lhs = self.clone();

        while lhs >= rhs {
            out = out + BigInt::of(1);
            lhs = lhs - rhs.clone();
        }
        out
    }
}

impl Display for BigInt {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        self.sanity_check();
        if self.digits.len() == 0 {
            write!(formatter, "0").expect("Failed to write string");
        } else {
            for d in self.digits.iter() {
                write!(formatter, "{}", d).expect("Failed to write string");
            }
        }
        Ok(())
    }
}

impl Sum<BigInt> for BigInt {
    fn sum<I>(iter: I) -> BigInt where I: Iterator<Item=BigInt> {
        iter.fold(BigInt::zero(), |acc, x| acc+x)
    }
}

pub fn big_factorial(n: BigInt) -> BigInt {
    if n == BigInt::zero() {
        BigInt::of(1)
    } else {
        n.clone() * big_factorial(n - BigInt::of(1))
    }
}

pub fn big_combination(n: BigInt, k: BigInt) -> BigInt {
    (big_factorial(n.clone()) / big_factorial(k.clone())) / big_factorial(n - k)
}

impl PartialEq<BigInt> for BigInt {
    fn eq(&self, other: &BigInt) -> bool {
        let max_length = max(self.digits.len(), other.digits.len());
        let self_length = self.digits.len();
        let other_length = other.digits.len();

        let zero = 0u8;
        let lhs = repeat(&zero).take(max_length - self_length).chain(self.digits.iter());
        let rhs = repeat(&zero).take(max_length - other_length).chain(other.digits.iter());

        lhs.zip(rhs).find(|&(x, y)| x != y).map_or(true, |_| false)
    }
}

impl Clone for BigInt {
    fn clone(&self) -> BigInt {
        BigInt { digits: self.digits.clone() }
    }
}

impl PartialOrd for BigInt {

    fn partial_cmp(&self, other: &BigInt) -> Option<Ordering> {

        let max_length = max(self.digits.len(), other.digits.len());

        let lhs = self.clone().zero_padded_to(max_length);
        let rhs = other.clone().zero_padded_to(max_length);

        format!("{}", lhs).partial_cmp(&format!("{}", rhs))
    }
}

//impl FromStr {
//}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_displays() {
        let x = BigInt::parse("53503534226472524250874054075591789781264330331690").unwrap();
        assert_eq!(format!("{}", x), "53503534226472524250874054075591789781264330331690")
    }

    #[test]
    fn big_decimal_adds() {
        let x = BigInt::parse("53503534226472524250874054075591789781264330331690").unwrap();
        let y = BigInt::parse("99999999999999999999999999999999999999999999999999").unwrap();

        let added = x + y;

        assert_eq!(format!("{}", added), "153503534226472524250874054075591789781264330331689")
    }

    #[test]
    fn big_decimal_divides() {
        let x = BigInt::parse("152343470").unwrap();
        let y = BigInt::parse("123455").unwrap();

        assert_eq!(format!("{}", x / y), "1234")
    }

    #[test]
    fn big_decimal_subtracts() {
        let x = BigInt::parse("13472515").unwrap();
        let y = BigInt::parse("123455").unwrap();

        let subtracted = x - y;

        assert_eq!(format!("{}", subtracted), "13349060");

        let x = BigInt::parse("370265").unwrap();
        let y = BigInt::parse("123455").unwrap();

        let subtracted = x - y;

        assert_eq!(format!("{}", subtracted), "246810")

    }

    #[test]
    fn big_decimal_multiplies() {
        let x = BigInt::parse("53503534226472524250874054075591789781264330331690").unwrap();
        let y = BigInt::parse("46376937677490009712648124896970078050417018260538").unwrap();

        let multiplied = x * y;
        let multiplied_expected = "2481330072346569912167630035632199641964226368882675583064264243801754492594498702941806796377849220";

        assert_eq!(format!("{}", multiplied), multiplied_expected)
    }

    #[test]
    fn big_factorial_works() {
        let fact = big_factorial(BigInt::of(34));
        assert_eq!("295232799039604140847618609643520000000", format!("{}", fact))
    }
}
