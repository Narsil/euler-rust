use crate::digits::{digits, from_digits};
use crate::eleven_twenty::triangle;
use num_bigint::BigUint;
use num_traits::pow::Pow;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fs::read_to_string;
use std::iter::FromIterator;

fn look(values: &[u64], rest_indices: &[u64], indexes: &HashMap<u64, HashMap<u64, Vec<u64>>>) {
    if rest_indices.len() == 0 {
        let last_value = values.last().unwrap();
        let first_value = values.first().unwrap();
        if last_value % 100 == first_value / 100 {
            println!("{:?}", values);
            println!("Pb 61 {}", values.iter().sum::<u64>());
        }
    }

    let suffix = values.last().unwrap() % 100;

    for index in rest_indices {
        let numberals_index = indexes.get(&index).unwrap();
        if let Some(new_values) = numberals_index.get(&suffix) {
            for value in new_values {
                let mut subvalues: Vec<_> = values.iter().cloned().collect();
                subvalues.push(*value);
                let new_indices: Vec<_> = rest_indices
                    .iter()
                    .cloned()
                    .filter(|ind| ind != index)
                    .collect();
                look(&subvalues, &new_indices, &indexes);
            }
        }
    }
}

pub fn pb61() {
    let mut triangle_index: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut t: u64 = 1;
    let mut i = 1;
    while t < 10_000 {
        if t >= 1_000 {
            let entry = triangle_index
                .entry((t / 100).try_into().unwrap())
                .or_insert(vec![]);
            entry.push(t);
        }
        i += 1;
        t = i * (i + 1) / 2;
    }

    let mut square_index: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut t = 1;
    let mut i = 1;
    while t < 10_000 {
        if t >= 1_000 {
            let entry = square_index
                .entry((t / 100).try_into().unwrap())
                .or_insert(vec![]);
            entry.push(t);
        }
        i += 1;
        t = i * i;
    }

    let mut pentagonal_index: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut t = 1;
    let mut i = 1;
    while t < 10_000 {
        if t >= 1_000 {
            let entry = pentagonal_index
                .entry((t / 100).try_into().unwrap())
                .or_insert(vec![]);
            entry.push(t);
        }
        i += 1;
        t = i * (3 * i - 1) / 2;
    }

    let mut hexagonal_index: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut t = 1;
    let mut i = 1;
    while t < 10_000 {
        if t >= 1_000 {
            let entry = hexagonal_index
                .entry((t / 100).try_into().unwrap())
                .or_insert(vec![]);
            entry.push(t);
        }
        i += 1;
        t = i * (2 * i - 1);
    }

    let mut heptagonal_index: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut t = 1;
    let mut i = 1;
    while t < 10_000 {
        if t >= 1_000 {
            let entry = heptagonal_index
                .entry((t / 100).try_into().unwrap())
                .or_insert(vec![]);
            entry.push(t);
        }
        i += 1;
        t = i * (5 * i - 3) / 2;
    }

    let mut octogonal_index: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut t = 1;
    let mut i = 1;
    while t < 10_000 {
        if t >= 1_000 {
            let entry = octogonal_index
                .entry((t / 100).try_into().unwrap())
                .or_insert(vec![]);
            entry.push(t);
        }
        i += 1;
        t = i * (3 * i - 2);
    }
    let mut indexes: HashMap<u64, HashMap<u64, Vec<u64>>> = HashMap::new();
    indexes.insert(3, triangle_index);
    indexes.insert(4, square_index);
    indexes.insert(5, pentagonal_index);
    indexes.insert(6, hexagonal_index);
    indexes.insert(7, heptagonal_index);
    // We will index on ocotonals first, there are less values.
    // indexes.insert(8, octogonal_index);

    let indices = [3, 4, 5, 6, 7];
    for (_, octogonals) in &octogonal_index {
        for octogonal in octogonals {
            look(&[*octogonal], &indices, &indexes);
        }
    }
}

pub fn pb62() {
    let mut a = BigUint::from(1u64);
    let one = BigUint::from(1u64);
    let mut digit_index: HashMap<num::BigUint, (BigUint, u32)> = HashMap::new();
    loop {
        let cube = a.pow(3u32);
        let mut digits: Vec<_> = cube.to_radix_le(10).into_iter().collect();
        digits.sort();
        let number = BigUint::from_radix_le(&digits, 10).unwrap();
        // println!("cube {}, number {}", cube, number);
        let entry = digit_index.entry(number).or_insert((cube, 0));
        entry.1 += 1;
        if entry.1 == 5 {
            println!("Pb 62: {}", entry.0);
            return;
        }
        a += &one;
    }
}

fn big_digits(p: &BigUint) -> Vec<u32> {
    p.to_radix_le(10).into_iter().map(|i| i as u32).collect()
}

pub fn pb63() {
    const LIMIT: usize = 200;
    let mut sum = 0;
    for a in (1..LIMIT).rev() {
        let aa = BigUint::from(a);
        let mut p = BigUint::from(a);
        for power in 1..LIMIT {
            let s: usize = big_digits(&p).len();
            if s == power {
                // println!("{}^{}", a, power);
                sum += 1;
            }
            p *= aa.clone();
        }
    }
    println!("Pb 63 {}", sum);
}

// x + n /  (sqrt(s) - d)
struct SqrtIterator {
    n: u64,
    d: u64,
    x: u64,
    s: u64,
}

impl SqrtIterator {
    pub fn new(s: u64) -> SqrtIterator {
        let d = (s as f64).sqrt() as u64;
        let n = 1;
        let x = d;
        SqrtIterator { n, d, s, x }
    }
}

impl Iterator for SqrtIterator {
    type Item = (u64, u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        let result = (self.x, self.n, self.d);
        // n / sqrt(s) - d
        // =  n * (sqrt(s) + d) / (s - d**2)
        // = X +  (sqrt(s) - d' / n')

        // n' = (s - d **2) / n  TODO prove it's divisible ?
        // X = integer_part((sqrt(s) + d ) / n')
        // n'  * X - d' = d
        // d' = n' * X - d;
        let nprime = (self.s - self.d.pow(2)) / self.n;
        if nprime == 0 {
            return None;
        }
        let x = (((self.s as f64).sqrt() + self.d as f64) / nprime as f64) as u64;
        let dprime = nprime * x - self.d;
        self.n = nprime;
        self.d = dprime;
        self.x = x;
        Some(result)
    }
}

fn sqrt_sequence(n: u64) -> SqrtIterator {
    SqrtIterator::new(n)
}

fn get_period(n: u64) -> u64 {
    let mut map: HashMap<(u64, u64, u64), u64> = HashMap::new();
    for (i, (x, n, d)) in sqrt_sequence(n).enumerate() {
        if let Some(j) = map.get(&(x, n, d)) {
            return i as u64 - j;
        }
        map.insert((x, n, d), i.try_into().unwrap());
    }
    0
}

pub fn pb64() {
    let mut sum = 0;
    for i in 2..=10_000 {
        let period = get_period(i);
        if period % 2 != 0 {
            sum += 1;
        }
    }
    println!("Pb 64 {}", sum);
}

fn continued_fraction(continued: impl DoubleEndedIterator<Item = u64>) -> (BigUint, BigUint) {
    let mut n = BigUint::from(1u32);
    let mut last_d: Option<BigUint> = None;
    for d in continued.rev() {
        if let Some(ld) = &last_d {
            let tmp = d * ld.clone() + n;
            n = ld.clone();
            last_d = Some(tmp);
        } else {
            n = BigUint::from(1u64);
            last_d = Some(BigUint::from(d))
        }
        // println!("{} / {:?}", n, last_d);
    }
    return (last_d.unwrap_or(n.clone()), n);
}

pub fn pb65() {
    let steps = 100;

    let continued = (1u64..=steps).map(|i| {
        if i == 1 {
            2
        } else if i % 3 == 0 {
            2 * i / 3
        } else {
            1
        }
    });
    let (_, last_d) = continued_fraction(continued);
    // 2
    // 2 + 1 / 1
    // 2 + 1 / ( 1 + 1/2 )
    // For steps = 3 -> 1/2 -> 3/2 -> 8/3
    // for i in (0..10_000).rev() {
    println!(
        "Pb 65 {}",
        last_d
            .to_radix_le(10)
            .into_iter()
            .map(|d| d as u32)
            .sum::<u32>()
    );
    // println!("{}/{}", last_d.unwrap(), n);
}

fn solutions(d: u64) -> (BigUint, BigUint) {
    let mut fractions: Vec<(u64, u64, u64)> = vec![];
    for (n, p, q) in sqrt_sequence(d) {
        let (a, b) = continued_fraction(fractions.iter().map(|(n, _p, _q)| *n));
        if &a * &a == &b * &b * d + BigUint::from(1u32) {
            return (a, b);
        }
        fractions.push((n, p, q));
    }
    panic!("Can't get here");
}

pub fn pb66() {
    let mut max = BigUint::from(0u32);
    let mut max_d = 0;
    let limit = 1_000;
    let squares: HashSet<u64> = HashSet::from_iter((2..limit).map(|i| i.pow(2u32)));
    for i in 2u64..=limit {
        // println!("i = {}", i);
        if squares.get(&i).is_some() {
            continue;
        }
        let (a, b) = solutions(i);
        // println!("{}^2 - {} * {}^2 = 1", a, i, b);
        if a > max {
            max = a;
            max_d = i;
        }
        if b > max {
            max = b;
            max_d = i;
        }
    }
    println!("Pb66 {}", max_d);
}
pub fn pb67() {
    let data = read_to_string("data/p067_triangle.txt").unwrap();
    println!("Pb67 {}", triangle(&data));
}

// Left for posterity, it's easier to read
#[allow(dead_code)]
fn gon_perm(p: [u8; 6]) -> Option<[u8; 9]> {
    // 0, 1, 2
    // 3, 2, 4
    // 5, 4, 1
    let a = p[0] + p[1] + p[2];
    let b = p[3] + p[2] + p[4];
    let c = p[5] + p[4] + p[1];
    if a == b && b == c {
        if p[0] < p[3] && p[0] < p[5] {
            Some([p[0], p[1], p[2], p[3], p[2], p[4], p[5], p[4], p[1]])
        } else if p[3] < p[0] && p[3] < p[5] {
            Some([p[3], p[2], p[4], p[5], p[4], p[1], p[0], p[1], p[2]])
        } else {
            Some([p[5], p[4], p[1], p[0], p[1], p[2], p[3], p[2], p[4]])
        }
    } else {
        None
    }
}

fn gon_perm10(p: [u8; 10]) -> Option<[u8; 15]> {
    // 0, 1, 2
    // 3, 2, 4
    // 5, 4, 6
    // 7, 6, 8
    // 9, 8, 1
    let a = p[0] + p[1] + p[2];
    let b = p[3] + p[2] + p[4];
    let c = p[5] + p[4] + p[6];
    let d = p[7] + p[6] + p[8];
    let e = p[9] + p[8] + p[1];
    if a == b && b == c && c == d && d == e {
        if p[0] < p[3] && p[0] < p[5] && p[0] < p[7] && p[0] < p[9] {
            Some([
                p[0], p[1], p[2], p[3], p[2], p[4], p[5], p[4], p[6], p[7], p[6], p[8], p[9], p[8],
                p[1],
            ])
        } else if p[3] < p[0] && p[3] < p[5] && p[3] < p[7] && p[3] < p[9] {
            Some([
                p[3], p[2], p[4], p[5], p[4], p[6], p[7], p[6], p[8], p[9], p[8], p[1], p[0], p[1],
                p[2],
            ])
        } else if p[5] < p[0] && p[5] < p[3] && p[5] < p[7] && p[5] < p[9] {
            Some([
                p[5], p[4], p[6], p[7], p[6], p[8], p[9], p[8], p[1], p[0], p[1], p[2], p[3], p[2],
                p[4],
            ])
        } else if p[7] < p[0] && p[7] < p[3] && p[7] < p[5] && p[7] < p[9] {
            Some([
                p[7], p[6], p[8], p[9], p[8], p[1], p[0], p[1], p[2], p[3], p[2], p[4], p[5], p[4],
                p[6],
            ])
        } else {
            Some([
                p[9], p[8], p[1], p[0], p[1], p[2], p[3], p[2], p[4], p[5], p[4], p[6], p[7], p[6],
                p[8],
            ])
        }
    } else {
        None
    }
}

// permutations!(6, permutations6, PermutationIterator);
permutations!(10, permutations10, PermutationIterator10);
pub fn pb68() {
    let mut max = 0;
    for permutation in permutations10([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]) {
        if let Some(gon) = gon_perm10(permutation) {
            let c = gon.iter().filter(|i| **i == 10).count();
            // Only 16-digit long
            if c == 1 {
                let n = from_digits(
                    &gon.iter()
                        .map(|n| if *n < 10 { vec![*n] } else { vec![1, 0] })
                        .flatten()
                        .collect::<Vec<_>>(),
                );
                if n > max {
                    max = n;
                }
            }
        }
    }
    println!("Pb68 {}", max);
}

pub fn pb69() {
    const LIMIT: usize = 1_000_000;
    let mut phi = [0; LIMIT];
    for i in 1..LIMIT {
        phi[i] = i;
    }
    let mut max = 0.0;
    let mut max_n = 0;
    for i in 2..LIMIT {
        if phi[i] == i {
            for j in (i..LIMIT).step_by(i) {
                phi[j] -= phi[j] / i;
            }
        }
        // println!("phi({}) = {}", i, phi[i]);
        let r = (i as f64) / phi[i] as f64;
        if r > max {
            max_n = i;
            max = r;
        }
    }
    println!("Pb69 {}", max_n);
}
pub fn pb70() {
    const LIMIT: usize = 10_000_000;
    let mut phi = vec![0; LIMIT];
    for i in 1..LIMIT {
        phi[i] = i;
    }
    let mut min = LIMIT as f64;
    let mut min_n = 0;
    for i in 2..LIMIT {
        if phi[i] == i {
            for j in (i..LIMIT).step_by(i) {
                phi[j] -= phi[j] / i;
            }
        }
        let mut digits_i: Vec<_> = digits(i as u64).collect();
        digits_i.sort();
        let mut digits_phi: Vec<_> = digits(phi[i] as u64).collect();
        digits_phi.sort();
        if digits_i.iter().eq(digits_phi.iter()) {
            let r = (i as f64) / phi[i] as f64;
            if r < min {
                min_n = i;
                min = r;
            }
        }
    }
    println!("Pb70 {}", min_n);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sqrt_iterator() {
        let mut it = sqrt_sequence(23);
        assert_eq!(it.next(), Some((4, 1, 4)));
        assert_eq!(it.next(), Some((1, 7, 3)));
        assert_eq!(it.next(), Some((3, 2, 3)));
        assert_eq!(it.next(), Some((1, 7, 4)));
        assert_eq!(it.next(), Some((8, 1, 4)));
        assert_eq!(it.next(), Some((1, 7, 3)));

        assert_eq!(get_period(23), 4);
        assert_eq!(get_period(2), 1);
        assert_eq!(get_period(3), 2);
        assert_eq!(get_period(4), 0);
        assert_eq!(get_period(5), 1);
        assert_eq!(get_period(13), 5);
    }
}
