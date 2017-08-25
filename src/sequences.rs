
use nonempty::NonEmptyVec;


pub struct IterativeSeq {
    initial: NonEmptyVec<i64>,
    f: fn(i64) -> i64,
}

pub fn collatz(first: i64) -> IterativeSeq {

    fn f(last: i64) -> i64 {
        if last % 2 == 0 { last / 2 } else { 3 * last + 1 }
    }

    IterativeSeq { initial: NonEmptyVec::singleton(first), f: f }
}

impl IterativeSeq {

    /// Length of the sequence from start till convergence (WARNING: Does not terminate if seq is non-converging)
    pub fn length(&self, convergence: i64) -> usize {
        self.iter().take_while(|&n| n != convergence).count()
    }

    pub fn nth(&self, n: usize) -> i64 {
        self.iter().nth(n).unwrap()
    }

    pub fn iter(&self) -> IterativeSeqIterator {
        IterativeSeqIterator { seq: self, last_initial_ix: None, last_value: None }
    }
}

struct IterativeSeqIterator<'a> {
    seq: &'a IterativeSeq,
    last_initial_ix: Option<usize>,
    last_value: Option<i64>,
}

impl <'a> Iterator for IterativeSeqIterator<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        let (next_last_initial_ix, next_last_value, n) =
            match (self.last_initial_ix, self.last_value) {
                (None, None) => (Some(0), None, self.seq.initial.head),
                (Some(ix), None) => {
                    let new_value = (self.seq.f)(self.seq.initial[ix]);
                    if (ix+1) == self.seq.initial.len() {
                        (None, Some(new_value), new_value)
                    } else {
                        (Some(ix+1), None, new_value)
                    }
                },
                (None, Some(last_value)) => {
                    let new_value = (self.seq.f)(last_value);
                    (None, Some(new_value), new_value)
                },
                (Some(_), Some(_)) => panic!("Inconsistent iterator state")
            };
        self.last_value = next_last_value;
        self.last_initial_ix = next_last_initial_ix;
        Some(n)
    }
}
