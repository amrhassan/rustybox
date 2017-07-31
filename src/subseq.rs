
use std::iter::Chain;
use std::iter::once;
use std::iter::Once;

/// An iterator from each element in the passed iterator
pub fn sub_iters<A, F : Iterator<Item=A> + Clone>(string: F)  -> SubIters<A, F> {
    SubIters(string.clone())
}

pub struct SubIters<A, F : Iterator<Item=A>>(F);

impl<A, F> Iterator for SubIters<A, F> where F : Iterator<Item=A> + Clone {

    type Item = Chain<Once<A>, F>;

    fn next(&mut self) -> Option<Chain<Once<A>, F>> {
        match self.0.next() {
            None => None,
            Some(h) => Some(once(h).chain(self.0.clone()))
        }
    }
}

/// Largest product of N adjacent entries in the given series
pub fn largest_product_in_series(series: Vec<u64>, n: usize) -> Option<u64> {
    sub_iters(series.iter())
        .map(|s| s.take(n).product())
        .max()
}
