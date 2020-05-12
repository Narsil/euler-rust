use num::pow::pow;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fs::read_to_string;

fn sum_divisors(i: u64, primes: &[u64]) -> u64 {
    let mut sum = 0;
    for divisor in crate::arithmetic::divisors(i, primes) {
        sum += divisor;
    }
    sum
}
pub fn pb21() {
    let mut sum = 0;
    let primes = crate::arithmetic::small_primes(10_000);
    for i in 2..10_000 {
        let b = sum_divisors(i, &primes);
        if b != i && sum_divisors(b, &primes) == i {
            sum += i;
        }
    }
    println!("Pb 21: {}", sum);
}
pub fn pb22() {
    let data = read_to_string("data/p022_names.txt").unwrap();

    let mut names = data
        .split(',')
        .map(|name_with_quote| name_with_quote.trim_matches('"'))
        .collect::<Vec<_>>();
    names.sort();

    fn score(name: &str) -> usize {
        // All caps and 'A' == 65
        name.chars().map(|c| (c as usize) - 64).sum()
    }

    let number: usize = names
        .iter()
        .enumerate()
        .map(|(i, name)| (i + 1) * score(name))
        .sum();
    println!("Pb 22: {}", number);
}

fn is_abundant(n: u64, primes: &[u64]) -> bool {
    sum_divisors(n, primes) > n
}

pub fn pb23() {
    const LIMIT: usize = 28124;
    let primes = crate::arithmetic::small_primes(30_000);
    let mut sieve = [false; LIMIT];
    let mut abundants: Vec<usize> = vec![];
    abundants.reserve(30_000);
    for i in 1..LIMIT {
        if is_abundant(i as u64, &primes) {
            abundants.push(i);
            for abundant in abundants.iter() {
                let s = i + abundant;
                if s < LIMIT {
                    sieve[i + abundant] = true;
                }
            }
        }
    }
    let mut sum = 0;
    for (n, is_sum_of_abundants) in sieve.iter().enumerate() {
        if !is_sum_of_abundants {
            // println!("N is not a sum of abundants {}", n);
            sum += n
        }
    }
    println!("Pb 23: {}", sum);
}

// https://www.quora.com/How-would-you-explain-an-algorithm-that-generates-permutations-using-lexicographic-ordering#
// Macro because array seems better than vec here (small array).
macro_rules! permutations {
    ($n: expr, $name: ident, $itname: ident) => {
        struct $itname {
            elements: [u8; $n],
            started: bool,
        }

        impl Iterator for $itname {
            type Item = [u8; $n];

            fn next(&mut self) -> Option<[u8; $n]> {
                // println!("---");
                if !self.started {
                    self.started = true;
                    return Some(self.elements);
                }
                // println!("elements {:?}", self.elements);
                #[inline]
                fn swap(i: usize, j: usize, array: &mut [u8; $n]) {
                    let tmp = array[i];
                    array[i] = array[j];
                    array[j] = tmp;
                };

                // 1. Find Find the largest x such that P[x]<P[x+1].
                // (If there is no such x, P is the last permutation.)
                for i in 0..$n - 1 {
                    let x = $n - 2 - i;
                    // println!("{} cmp {}", self.elements[x], self.elements[x + 1]);
                    if self.elements[x] < self.elements[x + 1] {
                        // println!("X = {} (val = {})", x, self.elements[x]);
                        // 2. Find the largest y such that P[x]<P[y].
                        for k in 0..$n - x - 1 {
                            let y = $n - 1 - k;
                            if self.elements[y] > self.elements[x] {
                                // println!("Y = {} (val = {})", y, self.elements[y]);
                                // 3. Swap P[x] and P[y].
                                swap(x, y, &mut self.elements);
                                // 4. Reverse P[x+1 .. n].
                                // println!("elts {:?}", self.elements);
                                for i in x + 1..$n {
                                    let j = $n - 1 - (i - (x + 1));
                                    if j <= i {
                                        break;
                                    }
                                    // println!("SWAP ({}, {})", i, j);
                                    swap(i, j, &mut self.elements);
                                }
                                return Some(self.elements);
                            }
                        }
                        panic!("We should never arrive here !");
                    }
                }
                return None;
            }
        }
        fn $name(elements: [u8; $n]) -> $itname {
            $itname {
                elements,
                started: false,
            }
        }
    };
}

permutations!(10, permutations, PermIterator);

pub fn pb24() {
    let digits: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for (i, iteration) in permutations(digits).enumerate() {
        if i == 1_000_000 - 1 {
            println!(
                "Pb 24: {}",
                iteration
                    .iter()
                    .map(|dig| dig.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            );
            break;
        }
    }
}

struct Fibonnaci {
    a: num_bigint::BigUint,
    b: num_bigint::BigUint,
}

impl Fibonnaci {
    pub fn new() -> Fibonnaci {
        let zero = num_bigint::BigUint::from(0u64);
        Fibonnaci {
            a: zero.clone(),
            b: zero,
        }
    }
}

impl Iterator for Fibonnaci {
    type Item = num_bigint::BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        let one = num_bigint::BigUint::from(1u64);
        let zero = num_bigint::BigUint::from(0u64);
        if self.a == zero {
            self.a = one.clone();
            Some(one)
        } else if self.b == zero {
            self.b = one.clone();
            Some(one)
        } else {
            let result = self.a.clone() + self.b.clone();
            self.a = self.b.clone();
            self.b = result.clone();
            Some(result)
        }
    }
}

pub fn pb25() {
    for (i, n) in Fibonnaci::new().enumerate() {
        if n.to_string().chars().count() >= 1_000 {
            println!("Pb 25: {}", i + 1);
            break;
        }
    }
}

fn recurring_cycle(divisor: u32) -> u32 {
    let mut dividend = 1;
    let mut seen: HashMap<u32, u32> = HashMap::new();
    let mut digits = vec![];
    let mut i = 0;
    loop {
        while dividend < divisor {
            seen.insert(dividend, i);
            digits.push(0);
            i += 1;
            dividend *= 10;
        }
        let quotient = dividend / divisor;
        // println!(
        //     "D {}, d {}, q {}, r {}",
        //     dividend,
        //     divisor,
        //     quotient,
        //     dividend % divisor
        // );
        let rest = dividend % divisor;
        digits.push(quotient);
        if let Some(j) = seen.get(&rest) {
            // println!("1/{} = {:?}[{}]", divisor, digits, i - j);
            return i - j;
        }
        seen.insert(rest, i);
        i += 1;
        if rest == 0 {
            break;
        }
        dividend = rest * 10;
    }
    // println!("1/{} = {:?}", divisor, digits);
    0
}

pub fn pb26() {
    let mut max = 0;
    let mut max_v = 0;
    for i in 2..1_000 {
        let recurring_cycle = recurring_cycle(i);
        if recurring_cycle > max {
            max = recurring_cycle;
            max_v = i;
        }
    }
    println!("Pb 26: {}", max_v);
}

fn num_primes(a: i32, b: i32, primes: &[u64]) -> i32 {
    for n in 0..1_000 {
        let x = n * n + a * n + b;
        // We should not exceed capacity of is_prime
        // and x is positive because of first condition
        if x < 2 || !crate::arithmetic::is_prime(x.try_into().unwrap(), primes).unwrap() {
            return n;
        }
    }
    panic!("We exceeded capacity !");
}

pub fn pb27() {
    let mut max = 0;
    let mut max_v = 0;
    let primes = crate::arithmetic::small_primes(1_000_000);
    // println!("num_primes {}", num_primes(1, 41, &primes));
    for a in -999..1_000 {
        for b in 0..=1_000 {
            let n = num_primes(a, b, &primes);
            if n > max {
                max = n;
                max_v = a * b;
            }
        }
    }
    println!("Pb 27: {}", max_v);
}
pub fn pb28() {
    let n = 1001;
    let mut side = n;
    let mut number = n * n;
    let mut sum = 1;
    // Spiral inwards
    while side > 1 && number > 1 {
        for _ in 0..4 {
            // println!("n {}", number);
            sum += number;
            number -= side - 1;
        }
        side -= 2;
        number = side * side;
    }
    println!("Pb 28: {}", sum);
}

pub fn pb29() {
    let mut results: HashSet<num_bigint::BigUint> = HashSet::new();
    let n = 100;
    for a in 2..=n {
        for b in 2..=n {
            let ap = num_bigint::BigUint::from(a);
            let c = pow(ap, b);
            results.insert(c);
        }
    }
    println!("Pb 29: {}", results.len());
}

pub fn pb30() {
    let mut sum = 0;
    for i in 2..1_000_000 {
        let digits: u64 = i
            .to_string()
            .chars()
            .map(|c| {
                let d: u64 = c.to_digit(10).unwrap().try_into().unwrap();
                d.pow(5)
            })
            .sum();
        if digits == i {
            sum += i;
        }
    }
    println!("Pb 30: {}", sum);
}

#[cfg(test)]
mod tests {

    permutations!(3, permutations3, PermIterator3);
    permutations!(4, permutations4, PermIterator4);

    #[test]
    fn test_permutations() {
        let mut per = permutations3([0, 1, 2]);
        assert_eq!(per.next(), Some([0, 1, 2]));
        assert_eq!(per.next(), Some([0, 2, 1]));
        assert_eq!(per.next(), Some([1, 0, 2]));
        assert_eq!(per.next(), Some([1, 2, 0]));
        assert_eq!(per.next(), Some([2, 0, 1]));
        assert_eq!(per.next(), Some([2, 1, 0]));
        assert_eq!(per.next(), None);
    }

    #[test]
    fn test_permutations4() {
        let mut per = permutations4([0, 1, 2, 3]);
        assert_eq!(per.next(), Some([0, 1, 2, 3]));
        assert_eq!(per.next(), Some([0, 1, 3, 2]));
        assert_eq!(per.next(), Some([0, 2, 1, 3]));
        assert_eq!(per.next(), Some([0, 2, 3, 1]));
        assert_eq!(per.next(), Some([0, 3, 1, 2]));
        assert_eq!(per.next(), Some([0, 3, 2, 1]));
        assert_eq!(per.next(), Some([1, 0, 2, 3]));
        assert_eq!(per.next(), Some([1, 0, 3, 2]));
        assert_eq!(per.next(), Some([1, 2, 0, 3]));
        assert_eq!(per.next(), Some([1, 2, 3, 0]));
        assert_eq!(per.next(), Some([1, 3, 0, 2]));
        assert_eq!(per.next(), Some([1, 3, 2, 0]));
        assert_eq!(per.next(), Some([2, 0, 1, 3]));
        assert_eq!(per.next(), Some([2, 0, 3, 1]));
        assert_eq!(per.next(), Some([2, 1, 0, 3]));
        assert_eq!(per.next(), Some([2, 1, 3, 0]));
        assert_eq!(per.next(), Some([2, 3, 0, 1]));
        assert_eq!(per.next(), Some([2, 3, 1, 0]));
        assert_eq!(per.next(), Some([3, 0, 1, 2]));
        assert_eq!(per.next(), Some([3, 0, 2, 1]));
        assert_eq!(per.next(), Some([3, 1, 0, 2]));
        assert_eq!(per.next(), Some([3, 1, 2, 0]));
        assert_eq!(per.next(), Some([3, 2, 0, 1]));
        assert_eq!(per.next(), Some([3, 2, 1, 0]));
        assert_eq!(per.next(), None);
    }
}
