

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
        self.iter().take_while(|&n| n != convergence).count()
    }

    pub fn iter(&self) -> IterativeSeqIterator {
        IterativeSeqIterator { seq: self, last: self.first }
    }
}

struct IterativeSeqIterator<'a> {
    seq: &'a IterativeSeq,
    last: u64,
}

impl <'a> Iterator for IterativeSeqIterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.last = (self.seq.f)(self.last);
        Some(self.last)
    }
}
