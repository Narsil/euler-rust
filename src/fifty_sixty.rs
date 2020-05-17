use crate::digits::{digits, from_digits, is_palindrome, Digit, Index, Number};
use itertools::structs::Combinations;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fs::read_to_string;
use std::iter::FromIterator;

struct PermutationIterator {
    iterator: Combinations<<std::vec::Vec<Index> as IntoIterator>::IntoIter>,
}

impl PermutationIterator {
    fn new(digit: Digit, n: Number, take_n: usize) -> PermutationIterator {
        let indices = digits(n)
            .enumerate()
            .filter(|(_i, d)| *d == digit)
            .map(|(i, _d)| i)
            .collect::<Vec<_>>();
        // println!("N {}, digit {}, indices {:?} ,", n, digit, indices);
        PermutationIterator {
            iterator: indices.into_iter().combinations(take_n),
        }
    }
}

impl Iterator for PermutationIterator {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(p) = self.iterator.next() {
            let n: u64 = p.iter().fold(0u64, |sum, i| {
                // println!("n {}, sum {}", i, sum);
                sum + 10u64.pow((*i).try_into().unwrap()) as u64
            });
            if n > 0 {
                Some(n)
            } else {
                None
            }
        } else {
            None
        }
    }
}

struct StepIterator {
    n: Number,
    pub take_n: usize,
    pub digit: Digit,
    iterator: PermutationIterator,
}

impl StepIterator {
    pub fn new(n: u64) -> StepIterator {
        let digit = 0;
        let take_n: usize = 2;
        let iterator = PermutationIterator::new(digit, n, take_n);
        StepIterator {
            n,
            digit,
            take_n,
            iterator,
        }
    }

    fn get_next_iterator(&mut self) -> Option<PermutationIterator> {
        self.take_n += 1;
        let digs = digits(self.n);
        let n_digits = digs.filter(|d| *d == self.digit).count();
        // println!("Get next iterator");
        if self.take_n > n_digits.try_into().unwrap() {
            self.take_n = 2;
            self.digit += 1;
            if self.digit > 2 {
                return None;
            }
        }
        Some(PermutationIterator::new(self.digit, self.n, self.take_n))
    }
}

impl Iterator for StepIterator {
    type Item = (Digit, Number);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.next() {
            Some(n) => Some((2 - self.digit, n)),
            None => {
                let mut r = None;
                while r == None {
                    match self.get_next_iterator() {
                        Some(mut iterator) => r = iterator.next(),
                        None => {
                            return None;
                        }
                    }
                }
                Some((2 - self.digit, r.unwrap()))
            }
        }
    }
}

fn steps(n: u64) -> StepIterator {
    StepIterator::new(n)
}

pub fn pb51() {
    let primes = crate::arithmetic::small_primes(1_000_000);
    let primes_set: HashSet<u64> = HashSet::from_iter(primes.iter().cloned());
    let max_p = 1_000_000;

    // This small example should have worked. I simply missed 101_010........
    // let steps = [
    //     11, 101, 1001, 1_010, 1_100, 10_001, 10010, 10100, 11_000, 100_001, 100_010, 100_100,
    //     101_00, 110_00, 111, 1101, 1110, 11_001, 11_010, 11_100, 110_001, 110_010, 110_100,
    //     111_000,
    // ];
    for p in primes {
        if p > max_p {
            break;
        }

        // We KNOW for sure we need only to check those numbers.
        // Because the family is 8 / 10 possibilities, we NEED to start with 0, 1 or 2 at worst.
        let digs = digits(p).collect::<Vec<_>>();
        if bytecount::naive_count(&digs, 0) < 3
            && bytecount::naive_count(&digs, 1) < 3
            && bytecount::naive_count(&digs, 2) < 3
        {
            continue;
        }

        // We iterate over digit, 0, 1 and 2, and look at each combination of 2 or more of this
        // specific digit. max_strikes represent the number of remaining "prime miss" avaiable.
        // If we look for digit 0, to make an 8 member family we have 2 strikes at most.
        // It also changes the number of addition we can make
        // step is ALWAYS a number in the form 11 0111.000111.. Because that's what
        // we use to update digits.
        for (max_strikes, step) in steps(p) {
            if step > p {
                break;
            }
            let mut pp = p;
            let mut strike = 0;
            let mut is_candidate = true;
            for _ in 0..9 - 2 + max_strikes {
                pp += step;
                if pp > max_p || primes_set.get(&pp).is_none() {
                    strike += 1;
                    if strike > max_strikes {
                        is_candidate = false;
                        break;
                    }
                }
            }
            if is_candidate {
                // println!("p {} is candidate", p);
                println!("Pb 51 {}", p);
                // let mut pp = p;
                // for _ in 0..9 {
                //     pp += step;
                //     if !(pp > max_p || primes_set.get(&pp).is_none()) {
                //         println!("pp {} is also prime", pp);
                //     }
                // }
                return;
            }
        }
    }
}

pub fn pb52() {
    for n in 1..1_000_000 {
        let digs: HashSet<Digit> = digits(n).collect();
        let mut stop = false;
        for k in 2..=6 {
            let b = k * n;
            let digs2: HashSet<Digit> = digits(b).collect();
            if digs != digs2 {
                stop = true;
                break;
            }
        }
        if stop {
            continue;
        }
        println!("Pb 52 {}", n);
        return;
    }
}
pub fn pb53() {
    // Remember C(n, k) = C(n -1, k) + C(n -1, k+1)
    // C(n, -1) = 1
    // C(n, n + 1) = 1
    // C(0, 0) = 1
    let mut pascal = vec![vec![1]];
    let limit = 100;
    let mut sum = 0;
    for n in 1..=limit {
        let mut row = vec![];
        for j in 0..n {
            let mut k = pascal[n - 1][j] + pascal[n - 1].get(j - 1).unwrap_or(&0);
            if k > 1_000_000 {
                sum += 1;
                k = 1_000_001;
            }
            row.push(k);
        }
        row.push(1);
        pascal.push(row);
    }
    // println!("Pascal {:?}", pascal);
    println!("Pb 53 {}", sum);
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Copy)]
enum Value {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Value {
    pub fn new(c: char) -> Option<Value> {
        match c {
            '2' => Some(Value::Two),
            '3' => Some(Value::Three),
            '4' => Some(Value::Four),
            '5' => Some(Value::Five),
            '6' => Some(Value::Six),
            '7' => Some(Value::Seven),
            '8' => Some(Value::Eight),
            '9' => Some(Value::Nine),
            'T' => Some(Value::Ten),
            'J' => Some(Value::Jack),
            'Q' => Some(Value::Queen),
            'K' => Some(Value::King),
            'A' => Some(Value::Ace),
            _ => None,
        }
    }
}

#[derive(PartialEq, Clone, Debug, Eq, Hash, Copy)]
enum Color {
    Spades,
    Clubs,
    Heart,
    Diamond,
}

impl Color {
    pub fn new(c: char) -> Option<Color> {
        match c {
            'H' => Some(Color::Heart),
            'D' => Some(Color::Diamond),
            'C' => Some(Color::Clubs),
            'S' => Some(Color::Spades),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
struct Card {
    value: Value,
    color: Color,
}

impl Ord for Card {
    fn cmp(&self, other: &Card) -> Ordering {
        self.value.cmp(&other.value)
    }
}
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
impl Eq for Card {}

impl PartialEq for Card {
    fn eq(&self, other: &Card) -> bool {
        self.value == other.value && self.color == other.color
    }
}

fn parse_cards(string: &str) -> Card {
    if string.chars().count() != 2 {
        panic!("Can't parse this {:?}", string);
    }
    let mut c = string.chars();
    let value = Value::new(c.next().unwrap()).unwrap();
    let color = Color::new(c.next().unwrap()).unwrap();
    Card { color, value }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct Cards {
    cards: Vec<Card>,
}

impl Cards {
    pub fn new(cards: Vec<Card>) -> Cards {
        Cards { cards }
    }

    fn is_flush(&self) -> bool {
        let colors: HashSet<Color> = self.cards.iter().map(|c| c.color).collect();
        colors.len() == 1
    }
    fn is_straight(&self) -> bool {
        // Special case for A, 5, 4, 3, 2
        if self.cards.iter().map(|c| c.value).collect::<Vec<_>>()
            == vec![
                Value::Ace,
                Value::Five,
                Value::Four,
                Value::Three,
                Value::Two,
            ]
        {
            return true;
        }

        let mut last_value = self.cards[0].value as u32;
        // println!("Last value {}", last_value);
        for c in self.cards.iter().skip(1) {
            let c_value = c.value as u32;
            if c_value != last_value - 1 && c_value != 14 && last_value != 2 {
                return false;
            }
            last_value = c_value;
        }
        true
    }

    fn groups(&self) -> HashMap<Value, u8> {
        let mut map: HashMap<Value, u8> = HashMap::new();
        for c in &self.cards {
            let entry = map.entry(c.value).or_insert(0);
            *entry += 1;
        }
        map
    }

    fn sort(&mut self) {
        let groups = self.groups();
        let mut count_cards = self
            .cards
            .iter()
            .cloned()
            .map(|c| (groups.get(&c.value).unwrap(), c))
            .collect::<Vec<_>>();
        count_cards.sort_by(|a, b| b.cmp(a));
        // count_cards.reverse();
        self.cards = count_cards.into_iter().map(|(_count, card)| card).collect();
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Hand {
    High(Cards),
    Pair(Cards),
    DoublePair(Cards),
    Three(Cards),
    Straight(Cards),
    Flush(Cards),
    FullHouse(Cards),
    Four(Cards),
    StraigtFlush(Cards),
}

fn parse_hand(mut cards: Cards) -> Option<Hand> {
    cards.sort();
    let groups = cards.groups();
    if cards.is_flush() && cards.is_straight() {
        Some(Hand::StraigtFlush(cards))
    } else if groups.values().any(|v| v == &4) {
        Some(Hand::Four(cards))
    } else if groups.values().any(|v| v == &3) && groups.values().any(|v| v == &2) {
        Some(Hand::FullHouse(cards))
    } else if cards.is_flush() {
        Some(Hand::Flush(cards))
    } else if cards.is_straight() {
        Some(Hand::Straight(cards))
    } else if groups.values().any(|v| v == &3) {
        Some(Hand::Three(cards))
    } else if groups.values().filter(|v| **v == 2).count() == 2 {
        Some(Hand::DoublePair(cards))
    } else if groups.values().any(|v| v == &2) {
        Some(Hand::Pair(cards))
    } else {
        Some(Hand::High(cards))
    }
}

pub fn pb54() {
    let data = read_to_string("data/p054_poker.txt").unwrap();
    let sum: u64 = data
        .trim()
        .split('\n')
        .map(|line| {
            if line.chars().count() == 0 {
                return 0;
            }
            let hands = line.trim().split(' ').map(parse_cards).collect::<Vec<_>>();
            let left = parse_hand(Cards::new(hands[..5].to_vec())).unwrap();
            let right = parse_hand(Cards::new(hands[5..].to_vec())).unwrap();
            // println!("Player 1 {:?}  vs Player2 {:?}", left, right);
            if left > right {
                1
            } else {
                0
            }
        })
        .sum();

    println!("Pb 54 {}", sum);
}

pub fn pb55() {
    let mut sum = 0;
    for i in 1u64..10_000 {
        let mut n = i;
        let mut is_lyschel = true;
        for _ in 0..50 {
            let mut digs = digits(n).collect::<Vec<_>>();
            n = from_digits(&digs);
            digs.reverse();
            let s = n.checked_add(from_digits(&digs));
            if s.is_none() {
                is_lyschel = true;
                break;
            }
            n = s.unwrap();
            if is_palindrome(n) {
                is_lyschel = false;
                break;
            }
        }
        if is_lyschel {
            sum += 1;
        }
    }
    println!("Pb 55 {}", sum);
}

fn big_digits(p: &num_bigint::BigUint) -> Vec<u32> {
    p.to_radix_le(10).into_iter().map(|i| i as u32).collect()
}

pub fn pb56() {
    const LIMIT: usize = 100;
    let mut max = 0;
    for a in (2..LIMIT).rev() {
        let aa = num_bigint::BigUint::from(a);
        let mut p = num_bigint::BigUint::from(a);
        for _ in 2..LIMIT {
            p *= aa.clone();
            let s: u32 = big_digits(&p).iter().sum();

            if s > max {
                max = s;
            }
        }
    }
    println!("Pb 56 {}", max);
}

struct FractionIterator {
    n: num_bigint::BigUint,
    d: num_bigint::BigUint,
}

impl Iterator for FractionIterator {
    type Item = (num_bigint::BigUint, num_bigint::BigUint);
    fn next(&mut self) -> Option<Self::Item> {
        let n = self.n.clone();
        let d = self.d.clone();
        self.d = d.clone() + n.clone();
        self.n = d.clone() + d + n;
        Some((self.n.clone(), self.d.clone()))
    }
}

fn fractions() -> FractionIterator {
    FractionIterator {
        n: num_bigint::BigUint::from(3u32),
        d: num_bigint::BigUint::from(2u32),
    }
}

pub fn pb57() {
    let mut sum = 0;
    for (numerator, denominator) in fractions().take(1_000 - 1) {
        // println!("{} / {}", numerator, denominator);
        if big_digits(&numerator).len() > big_digits(&denominator).len() {
            sum += 1;
        }
    }
    println!("Pb 57 {}", sum);
}

pub fn pb58() {
    let mut side = 4;
    let mut number = 9u64;
    let primes = crate::arithmetic::small_primes(1_000_000);
    let primes_set: HashSet<u64> = HashSet::from_iter(primes.iter().cloned());
    let mut total = 5f64;
    let mut n_primes = 3f64;
    while n_primes / total > 0.1 {
        for _ in 0..4 {
            number += side;
            // Show be miller rabin is prime for better speed !
            if primes_set.get(&number).is_some()
                || crate::arithmetic::is_prime(number, &primes).unwrap()
            {
                n_primes += 1.0;
            }
            total += 1.0;

            if number > 1_000_000u64 * 1_000_000u64 {
                panic!("Out of bounds !");
            }
        }
        side += 2;
    }
    println!("Pb 58 {}", side - 1);
}

fn decrypt(string: &[u8], i: u8, j: u8, k: u8) -> Vec<u8> {
    let mut v = vec![];
    v.reserve(string.len());
    let chunks = string.chunks(3);
    for chunk in chunks {
        // println!("Chunk {:?}", chunk);
        if let Some(x) = chunk.get(0) {
            v.push(x ^ i);
        }
        if let Some(x) = chunk.get(1) {
            v.push(x ^ j);
        }
        if let Some(x) = chunk.get(2) {
            v.push(x ^ k);
        }
    }
    v
}

pub fn pb59() {
    let string = read_to_string("data/p059_cipher.txt")
        .unwrap()
        .split(',')
        .map(|number| number.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    // println!("String {:?}", string);
    let mut max = 0;
    let mut key = (0, 0, 0);
    for i in 0..255u8 {
        for j in 0..255u8 {
            for k in 0..255u8 {
                // Only short amount of text required.
                let decrypted = decrypt(&string[..1_000], i, j, k);
                // 101 = 'e'
                // 32 =  ' '
                let count = bytecount::naive_count(&decrypted, 32);
                if count > max {
                    max = count;
                    key = (i, j, k);
                }
            }
        }
    }
    let msg = decrypt(&string[..], key.0, key.1, key.2);
    // let string = from_utf8(&msg[..]).unwrap();
    // println!("Decrypted {:?}", string);

    println!("Pb 59 {}", msg.iter().map(|c| *c as u32).sum::<u32>());
}
pub fn pb60() {
    let primes: Vec<u64> = crate::arithmetic::small_primes(100_000_000);
    let primes_set: HashSet<u64> = HashSet::from_iter(primes.iter().cloned());
    let mut set: Vec<u64> = vec![3u64];
    let mut indices: Vec<usize> = vec![1];
    let mut digits_set: Vec<Vec<Digit>> = set.iter().cloned().map(to_digits).collect();
    let mut max_p_index = 1;

    fn to_digits(n: u64) -> Vec<Digit> {
        digits(n).in_order()
    }
    while set.len() < 5 {
        let mut found = false;
        for (i, p) in primes[max_p_index + 1..].iter().enumerate() {
            let digs = to_digits(*p);
            // If digits are too long, there is no way we can check they are primes
            // fast enough. Probability the resulting sum will be the lowest
            // is *very* low.
            if digits_set.last().is_some() && digs.len() + digits_set.last().unwrap().len() > 8 {
                found = false;
                break;
            }

            let mut stop = false;
            for digs2 in &digits_set {
                // [A, B]
                let new_digits: Vec<_> =
                    digs.iter().cloned().chain(digs2.iter().cloned()).collect();
                let n = from_digits(&new_digits);
                if primes_set.get(&n).is_none() {
                    stop = true;
                    break;
                }

                // [B, A]
                let new_digits: Vec<_> =
                    digs2.iter().cloned().chain(digs.iter().cloned()).collect();
                let n = from_digits(&new_digits);
                if primes_set.get(&n).is_none() {
                    stop = true;
                    break;
                }
            }
            if stop {
                continue;
            }

            max_p_index += i + 1;
            set.push(*p);
            digits_set.push(to_digits(*p));
            indices.push(max_p_index);
            // println!("Found it {}", p);
            found = true;
            break;
        }
        if !found {
            if set.is_empty() {
                panic!("Nothing found");
            }
            // +1 so we don't check again the same number
            max_p_index = *indices.last().unwrap() + 1;
            set.pop();
            digits_set.pop();
            indices.pop();
        }
        // if set.len() >= 4 {
        //     println!("Set {:?}", set);
        // }
    }
    println!("Pb 60 {}", set.iter().sum::<u64>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_iterator() {
        let mut p = PermutationIterator::new(0, 11_001, 2);
        assert_eq!(p.next(), Some(110));
        assert_eq!(p.next(), None);

        let mut p = PermutationIterator::new(0, 10_001, 2);
        assert_eq!(p.next(), Some(110));
        assert_eq!(p.next(), Some(1_010));
        assert_eq!(p.next(), Some(1_100));
        assert_eq!(p.next(), None);

        let mut p = PermutationIterator::new(0, 10_001, 3);
        assert_eq!(p.next(), Some(1_110));
        assert_eq!(p.next(), None);

        let mut p = PermutationIterator::new(1, 11_001, 2);
        assert_eq!(p.next(), Some(1_001));
        assert_eq!(p.next(), Some(10_001));
        assert_eq!(p.next(), Some(11_000));
        assert_eq!(p.next(), None);
    }

    #[test]
    fn test_step_iterator() {
        let mut p = StepIterator::new(101);
        assert_eq!(p.next(), Some((1, 101)));
        assert_eq!(p.next(), None);

        let mut p = StepIterator::new(1001);
        assert_eq!(p.next(), Some((2, 110)));
        assert_eq!(p.next(), Some((1, 1001)));
        assert_eq!(p.next(), None);

        let mut p = StepIterator::new(100122);
        assert_eq!(p.next(), Some((2, 011_000)));
        assert_eq!(p.next(), Some((1, 100_100)));
        assert_eq!(p.next(), Some((0, 000_011)));
        assert_eq!(p.next(), None);
    }

    #[test]
    fn test_poker() {
        // StraightFlush
        let cards = Cards::new(
            "AH KH QH JH TH"
                .split(' ')
                .map(parse_cards)
                .collect::<Vec<_>>(),
        );
        assert_eq!(parse_hand(cards.clone()), Some(Hand::StraigtFlush(cards)));

        // Four
        let cards = Cards::new(
            "AH AS AC AD TH"
                .split(' ')
                .map(parse_cards)
                .collect::<Vec<_>>(),
        );
        assert_eq!(parse_hand(cards.clone()), Some(Hand::Four(cards)));

        // Full House
        let cards = Cards::new(
            "AH AS AC TD TH"
                .split(' ')
                .map(parse_cards)
                .collect::<Vec<_>>(),
        );
        assert_eq!(parse_hand(cards.clone()), Some(Hand::FullHouse(cards)));

        // Flush
        let cards = Cards::new(
            "AH KH QH JH 9H"
                .split(' ')
                .map(parse_cards)
                .collect::<Vec<_>>(),
        );
        assert_eq!(parse_hand(cards.clone()), Some(Hand::Flush(cards)));

        // Straight
        let cards = Cards::new(
            "AH KH QH JH TD"
                .split(' ')
                .map(parse_cards)
                .collect::<Vec<_>>(),
        );
        assert_eq!(parse_hand(cards.clone()), Some(Hand::Straight(cards)));

        // Three
        let cards = Cards::new(
            "AH AC AS TH 9D"
                .split(' ')
                .map(parse_cards)
                .collect::<Vec<_>>(),
        );
        assert_eq!(parse_hand(cards.clone()), Some(Hand::Three(cards)));

        // Double Pair
        let cards = Cards::new(
            "AH AC TS TH 9D"
                .split(' ')
                .map(parse_cards)
                .collect::<Vec<_>>(),
        );
        assert_eq!(parse_hand(cards.clone()), Some(Hand::DoublePair(cards)));

        // Pair
        let cards = Cards::new(
            "AH AC JS TH 9D"
                .split(' ')
                .map(parse_cards)
                .collect::<Vec<_>>(),
        );
        assert_eq!(parse_hand(cards.clone()), Some(Hand::Pair(cards)));

        // High
        let cards = Cards::new(
            "AH QC JS TH 9D"
                .split(' ')
                .map(parse_cards)
                .collect::<Vec<_>>(),
        );
        assert_eq!(parse_hand(cards.clone()), Some(Hand::High(cards)));
    }

    #[test]
    fn test_hand_comparison() {
        let cards = parse_hand(Cards::new(
            "AH QC JS TH 9D"
                .split(' ')
                .map(parse_cards)
                .collect::<Vec<_>>(),
        ));
        let cards2 = parse_hand(Cards::new(
            "AC AH JC TC 9C"
                .split(' ')
                .map(parse_cards)
                .collect::<Vec<_>>(),
        ));
        assert!(cards < cards2 && cards2 > cards);

        let cards = parse_hand(Cards::new(
            "AH AC TS 9H 6D"
                .split(' ')
                .map(parse_cards)
                .collect::<Vec<_>>(),
        ));
        let cards2 = parse_hand(Cards::new(
            "AD JH JC TC TC"
                .split(' ')
                .map(parse_cards)
                .collect::<Vec<_>>(),
        ));
        assert!(cards < cards2 && cards2 > cards);
    }
}
