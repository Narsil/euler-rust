use std::convert::TryInto;

pub type Digit = u8;
pub type Index = usize;
pub type Number = u64;

pub struct DigitIterator {
    n: Number,
}

impl DigitIterator {
    pub fn in_order(self) -> Vec<Digit> {
        self.collect::<Vec<_>>()
            .iter()
            .cloned()
            .rev()
            .collect::<Vec<_>>()
    }
}

impl Iterator for DigitIterator {
    type Item = Digit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            None
        } else {
            let d = self.n % 10;
            self.n /= 10;
            Some(d.try_into().unwrap())
        }
    }
}

pub fn digits(n: Number) -> DigitIterator {
    DigitIterator { n }
}

pub fn from_digits(digits: &[Digit]) -> u64 {
    let mut pow = 1;
    let mut number = 0u64;
    for i in 0..digits.len() {
        let index = digits.len() - 1 - i;
        let d = digits[index];
        if d >= 10 {
            panic!("{} is not a digit !", d);
        }
        number += digits[index] as u64 * pow;
        pow *= 10;
    }
    number
}

pub fn is_palindrome(i: u64) -> bool {
    let digs = digits(i).collect::<Vec<_>>();
    digs.clone().into_iter().rev().eq(digs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        let mut digs = digits(1234);
        assert_eq!(digs.next(), Some(4));
        assert_eq!(digs.next(), Some(3));
        assert_eq!(digs.next(), Some(2));
        assert_eq!(digs.next(), Some(1));
        assert_eq!(digs.next(), None);

        let digs = digits(1234).in_order();
        assert_eq!(digs, vec![1, 2, 3, 4]);
    }
}
