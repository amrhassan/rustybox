
use std::ops::Add;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use std::iter::repeat;
use std::cmp::max;
use std::iter::Sum;

/// Arbitrarily-sized unsigned BCD integer, MSB first.
/// Not a very efficient implementation as each decimal digit is stored in a whole byte.
pub struct BigDecimal { bytes: Vec<u8> }

impl BigDecimal {

    pub fn parse(ns: &str) -> Option<BigDecimal> {
        let mut n = BigDecimal { bytes: Vec::new() };
        let mut all_digits = true;
        for c in ns.chars() {
            match c.to_digit(10) {
                Some(d) => n.bytes.push(d as u8),
                None => all_digits = false
            }
        }
        if all_digits {
            Some(n)
        } else { None }
    }
}

impl Add for BigDecimal {

    type Output = BigDecimal;

    fn add(self, other: BigDecimal) -> BigDecimal {
        let max_length = max(self.bytes.len(), other.bytes.len());
        let self_length = self.bytes.len();
        let other_length = other.bytes.len();

        let lhs = repeat(0u8).take(max_length - self_length).chain(self.bytes.into_iter());
        let rhs = repeat(0u8).take(max_length - other_length).chain(other.bytes.into_iter());

        let mut out: Vec<u8> = lhs.zip(rhs).map(|(x, y)| x + y).collect();

        // Makes sure every byte contains a single decimal digit
        let mut carry = 0;
        for ix in (0..out.len()).rev() {
            out[ix] = out[ix] + carry;
            carry = 0;
            if out[ix] > 9 {
                out[ix] = out[ix] % 10;
                carry = 1;
            }
        }
        if carry > 0 {
            out.insert(0, carry);
        }

        BigDecimal { bytes: out }
    }
}

impl Display for BigDecimal {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        if self.bytes.len() == 0 {
            write!(formatter, "0").expect("Failed to write string");
        } else {
            for d in self.bytes.iter() {
                write!(formatter, "{}", d).expect("Failed to write string");
            }
        }
        Ok(())
    }
}

impl Sum<BigDecimal> for BigDecimal {
    fn sum<I>(iter: I) -> BigDecimal where I: Iterator<Item=BigDecimal> {
        iter.fold(BigDecimal::zero(), |acc, x| acc+x)
    }
}

impl BigDecimal {
    fn zero() -> BigDecimal {
        BigDecimal { bytes: Vec::new() }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_displays() {
        assert_eq!(
            format!("{}", BigDecimal::parse("53503534226472524250874054075591789781264330331690").unwrap()),
            "53503534226472524250874054075591789781264330331690"
        )
    }

    #[test]
    fn big_decimal_parses_and_adds() {
        let x = BigDecimal::parse("53503534226472524250874054075591789781264330331690").unwrap();
        let y = BigDecimal::parse("46376937677490009712648124896970078050417018260538").unwrap();

        let added = x + y;

        assert_eq!(format!("{}", added), "99880471903962533963522178972561867831681348592228")
    }
}
