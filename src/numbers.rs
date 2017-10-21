
// 17

/// Letter count of number n such that 0 <= n <= 1000
pub fn number_letter_count(n: u32) -> u64 {

    let one_to_nineteen = |x| {
        slen(match x {
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            10 => "ten",
            11 => "eleven",
            12 => "twelve",
            13 => "thirteen",
            14 => "fourteen",
            15 => "fifteen",
            16 => "sixteen",
            17 => "seventeen",
            18 => "eighteen",
            19 => "nineteen",
            _ => ""
        })
    };

    if n < 20 {
        match n {
            0 => slen("zero"),
            _ => one_to_nineteen(n)
        }
    } else if n < 30 {
        slen("twenty") + one_to_nineteen(n-20)
    } else if n < 40 {
        slen("thirty") + one_to_nineteen(n-30)
    } else if n < 50 {
        slen("forty") + one_to_nineteen(n-40)
    } else if n < 60 {
        slen("fifty") + one_to_nineteen(n-50)
    } else if n < 70 {
        slen("sixty") + one_to_nineteen(n-60)
    } else if n < 80 {
        slen("seventy") + one_to_nineteen(n-70)
    } else if n < 90 {
        slen("eighty") + one_to_nineteen(n-80)
    } else if n < 100 {
        slen("ninety") + one_to_nineteen(n-90)
    } else if n < 1000 {
        let hundred_count = n / 100;
        let leftover = n % 100;

        one_to_nineteen(hundred_count) + slen("hundred") + (if leftover > 0 {
            slen("and") + number_letter_count(leftover)
        } else { 0 })
    } else {
        slen("onethousand")
    }
}

fn slen(s: &str) -> u64 {
    s.len() as u64
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example_counts() {
        assert_eq!(number_letter_count(342), 23);
        assert_eq!(number_letter_count(115), 20);
    }

}