pub struct DigitIterator {
    n: usize,
}

impl Iterator for DigitIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            None
        } else {
            let d = self.n % 10;
            self.n /= 10;
            Some(d)
        }
    }
}

pub fn digits(n: usize) -> DigitIterator {
    DigitIterator { n }
}

pub fn from_digits(digits: &[usize]) -> u64 {
    let mut pow = 1;
    let mut number = 0u64;
    for i in 0..digits.len() {
        let index = digits.len() - 1 - i;
        number += digits[index] as u64 * pow;
        pow *= 10;
    }
    number
}
