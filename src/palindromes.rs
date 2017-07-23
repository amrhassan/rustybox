
use std::fmt::Display;

fn is_palindrome<T : Eq + PartialEq>(ts: &[T]) -> bool {
    ts.iter().rev().eq(ts)
}

/// Checks if the display of a value is palindromic
fn is_palindrome_display<T : Display>(d: T) -> bool {
    is_palindrome(format!("{}", d).as_bytes())
}

fn products_of_n_digit_numbers(n: u32) -> Vec<u32> {
    let mut products = Vec::new();
    let base: u32 = 10;
    for x in 1..base.pow(n) {
        for y in 1..base.pow(n) {
            products.push(x*y);
        }
    }
    products
}

pub fn largest_palindrome_of_product_of_n_digit_numbers(n: u32) -> u32 {
    let mut palindromes: Vec<u32> =
        products_of_n_digit_numbers(n)
            .into_iter()
            .filter(|n| is_palindrome_display(n))
            .collect();

    palindromes.sort();
    *(palindromes.last().unwrap())
}
