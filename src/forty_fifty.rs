use crate::digits::{digits, from_digits, Digit};
use std::collections::HashSet;
use std::convert::TryInto;
use std::fs::read_to_string;
use std::iter::FromIterator;

// permutations!(9, permutations9, PermIterator9);
// permutations!(8, permutations8, PermIterator8);
permutations!(7, permutations7, PermIterator7, Digit);
// permutations!(6, permutations6, PermIterator6);
// permutations!(5, permutations5, PermIterator5);

pub fn pb41() {
    // Can't be 9 or 8 because 1 + 2 +.. 9 = 45, divisible by 9, 1+2+..8 = 36 divisible by 9.
    let primes: HashSet<u64> =
        HashSet::from_iter(crate::arithmetic::small_primes(10_000_000).into_iter());
    // for permutation in permutations9([1, 2, 3, 4, 5, 6, 7, 8, 9]) {
    //     // 2-N
    //     let pan_number = from_digits(
    //         &permutation
    //             .iter()
    //             .map(|i| (10 - i) as usize)
    //             .collect::<Vec<usize>>()[..],
    //     );
    //     if !primes.get(&pan_number).is_none() {
    //         println!("Pb41: {}", pan_number);
    //         return;
    //     }
    // }
    // for permutation in permutations8([1, 2, 3, 4, 5, 6, 7, 8]) {
    //     // 2-N
    //     let pan_number = from_digits(
    //         &permutation
    //             .iter()
    //             .map(|i| (9 - i) as usize)
    //             .collect::<Vec<usize>>()[..],
    //     );
    //     if !primes.get(&pan_number).is_none() {
    //         println!("Pb41: {}", pan_number);
    //         return;
    //     }
    // }
    for permutation in permutations7([1, 2, 3, 4, 5, 6, 7]) {
        // 2-N
        let pan_number = from_digits(&permutation.iter().map(|i| (8 - i)).collect::<Vec<_>>()[..]);
        if primes.get(&pan_number).is_some() {
            println!("Pb41: {}", pan_number);
            return;
        }
    }
}

pub fn pb42() {
    let triangle_numbers: HashSet<u32> = HashSet::from_iter((0u32..100).map(|i| i * (i + 1) / 2));
    let string = read_to_string("data/p042_words.txt").unwrap();
    let data = string.trim().split(',').map(|w| w.trim_matches('"'));
    let scores = data.map(|w| w.chars().map(|c| (c as u32) - 64).sum());
    let num = scores
        .filter(|s| triangle_numbers.get(&s).is_some())
        .count();
    println!("Pb42: {}", num);
}

permutations!(10, permutations10, PermIterator10, Digit);
pub fn pb43() {
    let mut sum = 0;
    for permutation in permutations10([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]) {
        if from_digits(&permutation[7..]) % 17 == 0
            && from_digits(&permutation[6..=8]) % 13 == 0
            && from_digits(&permutation[5..=7]) % 11 == 0
            && from_digits(&permutation[4..=6]) % 7 == 0
            && from_digits(&permutation[3..=5]) % 5 == 0
            && from_digits(&permutation[2..=4]) % 3 == 0
            && from_digits(&permutation[1..=3]) % 2 == 0
        {
            sum += from_digits(&permutation[..]);
        }
    }
    println!("Pb43: {}", sum);
}
pub fn pb44() {
    let mut pentagonals: HashSet<i32> = HashSet::new();
    let mut i = 1;
    loop {
        let pi = i * (3 * i - 1) / 2;
        for pk in &pentagonals {
            if let Some(pj) = pentagonals.get(&(pi - pk)) {
                let diff = (pj - pk).abs();
                if pentagonals.get(&diff).is_some() {
                    println!("Pb44: {}", diff);
                    return;
                }
            }
        }
        pentagonals.insert(pi);
        i += 1;
    }
}
pub fn pb45() {
    let mut i = 2u64;
    let mut j = 2;
    let mut k = 2;

    // (n + 1) (n + 2) / 2  - n (n+1) / 2 = [(n**2 + 3n + 2) - n **2 - n)] / 2
    // = (2n + 2) / 2 = n + 1

    let mut triangle = i * (i + 1) / 2;
    let mut pentagonal = j * (3 * j - 1) / 2;
    let mut hexagonal = k * (2 * k - 1);

    loop {
        // println!("t {}, p{} ,h{}", t, p, h);
        while triangle < pentagonal || triangle < hexagonal {
            i += 1;
            triangle += i;
        }
        while pentagonal < triangle || pentagonal < hexagonal {
            j += 1;
            pentagonal = j * (3 * j - 1) / 2;
        }
        while hexagonal < triangle || hexagonal < pentagonal {
            k += 1;
            hexagonal = k * (2 * k - 1);
        }
        if triangle == pentagonal && pentagonal == hexagonal && triangle != 40755 {
            println!("Pb45: {}", triangle);
            return;
        }
        i += 1;
        triangle += i;
    }
}
pub fn pb46() {
    const LIMIT: u32 = 10_000;
    let primes = crate::arithmetic::small_primes(LIMIT.into());
    let a_limit = 100;
    let mut sieve = [false; LIMIT as usize];
    for siev in sieve.iter_mut().take(9) {
        *siev = true;
    }
    let mut last_prime = 2;
    for prime in &primes {
        for i in last_prime + 1..*prime {
            if i % 2 == 0 {
                continue;
            }
            if !sieve[i as usize] {
                println!("Pb46: {}", i);
                return;
            }
        }
        for i in 0..a_limit {
            let n = prime + 2 * i * i;
            if n >= LIMIT.into() {
                break;
            }
            sieve[n as usize] = true;
        }
        last_prime = *prime;
    }
}
pub fn pb47() {
    let limit: usize = 1_000_000;
    let primes = crate::arithmetic::small_primes(limit as u64);
    let primes_set: HashSet<u64> = HashSet::from_iter(primes.iter().cloned());
    let mut previous = vec![];
    let n = 4;
    for i in 2..limit {
        // Speedup because prime_factors of large primes is slow.
        if primes_set.get(&i.try_into().unwrap()).is_some() {
            previous.clear();
            continue;
        }
        let factors = crate::arithmetic::prime_factors(i as u64, &primes);
        if factors.len() != n {
            previous.clear();
            continue;
        }
        previous.push(factors);
        if previous.len() == n {
            println!("Pb47: {}", i - (n - 1));
            return;
        }
    }
}

const LAST_TEN: u128 = 10_000_000_000;
fn get(i: u128, power: u128) -> u128 {
    if power == 1 {
        i
    } else if power % 2 == 0 {
        let r = get(i, power / 2);
        (r * r) % LAST_TEN
    } else {
        let r = get(i, (power - 1) / 2);
        (((r * r) % LAST_TEN) * i) % LAST_TEN
    }
}

pub fn pb48() {
    let mut sum = 0;
    // println!("{}", get(980, 980));
    for i in 1..=1000 {
        let res = get(i, i);
        // println!("i {}, {}", i, res);
        sum = (sum + res) % LAST_TEN
    }
    println!("Pb48: {}", sum);
}
pub fn pb49() {
    let primes = crate::arithmetic::small_primes(10_000);
    let primes_set: HashSet<u64> = HashSet::from_iter(primes.iter().cloned());
    for (i, p1) in (&primes).iter().enumerate() {
        if p1 < &1000 {
            continue;
        }
        if p1 == &1487 {
            continue;
        }
        let mut digs1 = digits(*p1).collect::<Vec<_>>();
        digs1.sort();
        for p2 in &primes[i + 1..] {
            let mut digs2 = digits(*p2).collect::<Vec<_>>();
            digs2.sort();
            if !digs1.iter().eq(digs2.iter()) {
                continue;
            }
            let p3 = p2 + (p2 - p1);
            if primes_set.get(&p3).is_none() {
                continue;
            }
            let mut digs3 = digits(p3).collect::<Vec<_>>();
            digs3.sort();
            if digs1.iter().eq(digs3.iter()) {
                println!("Pb49 {}{}{}", p1, p2, p3);
            }
        }
    }
}
pub fn pb50() {
    let limit = 1_000_000;
    let primes = crate::arithmetic::small_primes(limit);
    let primes_set: HashSet<u64> = HashSet::from_iter(primes.iter().cloned());
    let mut sum = 0;
    let mut max_len = 1;
    let mut max = 0;
    for (i, p) in primes.iter().enumerate() {
        sum += p;
        // Very important for speed, we know there is at least on 21 sequence long
        // So there is no way solution can be found above.
        // This is not entirely correct but this heuristic worked.
        if *p > limit / 21 {
            break;
        }
        if i < max_len {
            continue;
        }

        let mut sum2 = sum;

        for (j, p2) in primes[..=i].iter().enumerate() {
            let len = i - j;
            if len < max_len {
                break;
            }
            if primes_set.get(&sum2).is_some() && len > max_len {
                max = sum2;
                max_len = len;
            }
            sum2 -= p2;
        }
    }
    println!("Pb 50 {}", max);
}
