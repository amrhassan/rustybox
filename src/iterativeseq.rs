

pub struct IterativeSeq {
    pub first: u64,
    f: fn(u64) -> u64,
}

pub fn collatz(first: u64) -> IterativeSeq {

    fn f(last: u64) -> u64 {
        if last % 2 == 0 { last / 2 } else { 3 * last + 1 }
    }

    IterativeSeq { first: first, f: f }
}

impl IterativeSeq {

    /// Length of the sequence from start till convergence (WARNING: Does not terminate if seq is non-converging)
    pub fn length(&self, convergence: u64) -> usize {
        let mut i = self.first;
        let mut length = 1;
        while i != convergence {
            length += 1;
            i = (self.f)(i);
        }
        length
    }
}
