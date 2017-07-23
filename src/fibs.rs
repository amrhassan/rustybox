
// 2

pub fn sum_even_fibs_under_4_million() -> u32 {
    let upper = 4_000_000;

    let fibs = Fib::start_from(1, 2).take_while(|x| x < &upper);

    let even_fibs = fibs.filter(|x| x % 2 == 0);

    even_fibs.fold(0, |x, y| x + y)
}

struct Fib {a: u32, b: u32}

impl Fib {
    fn start_from(a: u32, b: u32) -> Fib {
        Fib {a: a, b: b}
    }
}

impl Iterator for Fib {

    type Item = u32;

    fn next(&mut self) -> Option<u32> {

        let next_val = self.a;
        let new_b = match (&self.a, &self.b) {
            (a, b) => a + b
        };

        self.a = self.b;
        self.b = new_b;

        Some(next_val)
    }
}

