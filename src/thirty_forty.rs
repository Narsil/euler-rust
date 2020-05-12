use num::integer::gcd;
use num::pow::pow;
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::TryInto;
use std::iter::FromIterator;

pub fn pb31() {
    let coins: [usize; 8] = [1, 2, 5, 10, 20, 50, 100, 200];
    const AMOUNT: usize = 200;
    let mut ways: [u64; AMOUNT + 1] = [0u64; AMOUNT + 1];

    ways[0] = 1;

    for coin in coins.iter() {
        for j in *coin..=AMOUNT {
            ways[j] = ways[j] + ways[j - coin];
        }
    }
    println!("Pb31 : {}", ways[AMOUNT]);
}

// permutations!(9, permutations9, PermIterator);

fn is_pandigital(mut a: u64) -> bool {
    let mut result: u64 = 0;
    while a > 0 {
        let digit = a % 10;
        if digit == 0 {
            return false;
        }
        result |= 1 << (digit - 1);
        a /= 10;
    }
    result == 0x1ff
}

fn log(n: u64) -> u32 {
    if n < 10 {
        1
    } else if n < 100 {
        2
    } else if n < 1_000 {
        3
    } else if n < 10_000 {
        4
    } else {
        panic!("Too big number");
    }
}

fn combine(a: u64, b: u64, c: u64) -> u64 {
    a + b * 10u64.pow(log(a)) + c * 10u64.pow(log(a) + log(b))
}

pub fn pb32() {
    // Kinda bruteforce solution.
    // let digits = [1u8, 2, 3, 4, 5, 6, 7, 8, 9];
    // let mut seen: HashSet<u32> = HashSet::new();
    // for permutation in permutations9(digits) {
    //     for j in 5..=5 {
    //         for k in 1..j {
    //             let a: u32 = permutation[0..k]
    //                 .iter()
    //                 .map(|d| *d as u32)
    //                 .enumerate()
    //                 .map(|(i, d)| 10u32.pow(i.try_into().unwrap()) * d)
    //                 .sum();
    //             let b: u32 = permutation[k..j]
    //                 .iter()
    //                 .map(|d| *d as u32)
    //                 .enumerate()
    //                 .map(|(i, d)| 10u32.pow(i.try_into().unwrap()) * d)
    //                 .sum();
    //             let c: u32 = permutation[j..]
    //                 .iter()
    //                 .map(|d| *d as u32)
    //                 .enumerate()
    //                 .map(|(i, d)| 10u32.pow(i.try_into().unwrap()) * d)
    //                 .sum();
    //             if a * b == c {
    //                 seen.insert(c);
    //             }
    //         }
    //     }
    // }
    // println!("Pb32: {}", seen.iter().sum::<u32>());

    // 8ms vs 1.3s
    let primes = crate::arithmetic::small_primes(1_000);
    let mut sum = 0;
    for c in 1_000..10_000 {
        for a in crate::arithmetic::divisors(c, &primes) {
            let b = c / a;
            if b < a {
                break;
            }
            let n = combine(a, b, c);
            if is_pandigital(n) {
                sum += c;
                // No need to find more multiplications
                break;
            }
        }
    }
    println!("Pb32: {}", sum);
}

pub fn pb33() {
    let mut fractions: Vec<(u32, u32)> = vec![];
    fractions.reserve(4);
    for i in 1..=9 {
        for j in 1..=9 {
            if i == j {
                continue;
            }
            for k in 1..=9 {
                let a = k * 10 + i;
                let b = j * 10 + k;
                let l = gcd(a, b);
                let m = gcd(i, j);
                if a < b && a / l == i / m && b / l == j / m {
                    fractions.push((i, j));
                }
                let a = i * 10 + k;
                let b = k * 10 + j;
                let l = gcd(a, b);
                let m = gcd(i, j);
                if a < b && a / l == i / m && b / l == j / m {
                    fractions.push((i, j));
                }
            }
        }
    }
    let (c, d) = fractions
        .iter()
        .fold((1, 1), |(a, b), (num, denom)| (a * num, b * denom));
    let sum = d / gcd(c, d);

    println!("Pb33: {}", sum);
}

struct DigitIterator {
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

fn digits(n: usize) -> DigitIterator {
    DigitIterator { n }
}

pub fn pb34() {
    let mut facto_digits = [0usize; 10];
    let mut fact: usize = 1;
    facto_digits[0] = 1;
    for i in 1..=9 {
        fact *= i;
        facto_digits[i] = fact;
    }

    let mut sum = 0;
    for i in 3..1_000_000 {
        let mut s = 0;
        for digit in digits(i) {
            s += facto_digits[digit];
        }
        if s == i {
            sum += i;
        }
    }

    println!("Pb34: {}", sum);
}

fn from_digits(digits: &[usize]) -> u64 {
    let mut pow = 1;
    let mut number = 0u64;
    for i in 0..digits.len() {
        let index = digits.len() - 1 - i;
        number += digits[index] as u64 * pow;
        pow *= 10;
    }
    number
}

pub fn pb35() {
    let mut sum = 0;
    let primes: HashSet<u64> =
        HashSet::from_iter(crate::arithmetic::small_primes(1_000_000).into_iter());
    for p in &primes {
        let mut digs = digits(*p as usize)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<VecDeque<_>>();
        let mut stop = false;
        for _ in 0..digs.len() {
            let f = digs.pop_front().unwrap();
            digs.push_back(f);
            let number = from_digits(&digs.clone().into_iter().collect::<Vec<_>>()[..]);
            if primes.get(&number).is_none() {
                stop = true;
                break;
            }
        }
        if !stop {
            sum += 1;
        }
    }
    println!("Pb35: {}", sum);
}

fn is_palindrome(i: usize) -> bool {
    let digs = digits(i).collect::<Vec<_>>();
    digs.clone().into_iter().rev().eq(digs)
}

fn binary_digits(mut i: usize) -> Vec<bool> {
    let mut vec = vec![];
    while i > 0 {
        let digit = (i & 1) != 0;
        vec.push(digit);
        i = i >> 1;
    }
    vec
}

fn is_binary_palindrome(i: usize) -> bool {
    let digs = binary_digits(i);
    digs.clone().iter().rev().eq(digs.iter())
}

pub fn pb36() {
    let mut sum = 0;
    // Generating palindromes would be faster.
    for i in 1..1_000_000 {
        if is_palindrome(i) && is_binary_palindrome(i) {
            sum += i;
        }
    }
    println!("Pb36: {}", sum);
}

pub fn pb37() {
    let primes: HashSet<u64> =
        HashSet::from_iter(crate::arithmetic::small_primes(1_000_000).into_iter());
    let mut sum = 0;
    for p in &primes {
        let digs = digits((*p).try_into().unwrap())
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>();
        if digs.len() == 1 {
            continue;
        }
        let mut stop = false;
        for i in 1..digs.len() {
            if primes.get(&from_digits(&digs[0..i])).is_none() {
                stop = true;
                break;
            }

            if primes.get(&from_digits(&digs[i..])).is_none() {
                stop = true;
                break;
            }
        }
        if !stop {
            sum += p;
        }
    }
    println!("Pb37: {}", sum);
}

permutations!(9, permutations9, PermIterator9);

pub fn pb38() {
    for permutation in permutations9([1, 2, 3, 4, 5, 6, 7, 8, 9]) {
        // 2-N
        let digits = permutation
            .iter()
            .map(|i| (10 - i) as usize)
            .collect::<Vec<usize>>();
        let left = from_digits(&digits[..4]);
        let right = from_digits(&digits[4..]);
        // Incomplete solution but works with only 2 parts ! Yeah
        if left * 2 == right {
            println!("Pb38: {}", from_digits(&digits));
            break;
        }
    }
}
pub fn pb39() {
    let mut triplets: HashMap<u32, u32> = HashMap::new();
    for a in 1..500 {
        for b in a..500 {
            let c2 = a * a + b * b;
            let c = (c2 as f64).sqrt().floor() as u32;
            if pow(c, 2) == c2 {
                let counter = triplets.entry(a + b + c).or_insert(0);
                *counter += 1;
            }
        }
    }
    let mut max = 0;
    let mut max_v = 0;
    for (triplet, count) in triplets {
        if count > max {
            max = count;
            max_v = triplet
        }
    }
    println!("Pb39: {}", max_v);
}

struct IrrDigits {
    n: u32,
    digits: Vec<u8>,
}

impl IrrDigits {
    fn new() -> IrrDigits {
        IrrDigits {
            n: 0,
            digits: vec![0],
        }
    }
}

impl Iterator for IrrDigits {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.digits.is_empty() {
            self.n += 1;
            self.digits = digits(self.n as usize).map(|i| i as u8).collect::<Vec<_>>()
        }
        self.digits.pop()
    }
}

pub fn pb40() {
    let mut prod = 1;
    let values = [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000];
    for (i, d) in IrrDigits::new().enumerate() {
        if values.contains(&i) {
            prod *= d;
        }
        if i > 1_000_000 {
            break;
        }
    }
    println!("Pb40: {}", prod);
}
