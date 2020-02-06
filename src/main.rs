use std::cmp::max;
use std::collections::HashMap;

mod arithmetic;
use arithmetic::{small_primes, small_primes_nth};

fn pb1() {
    let mut sum = 0;
    for i in 0..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i
        }
    }
    println!("Pb1: {}", sum)
}

fn pb2() {
    let mut sum = 0;
    let mut a = 0;
    let mut b = 1;
    while b < 4_000_000 {
        let c = a + b;
        if c % 2 == 0 {
            sum += c
        }
        a = b;
        b = c;
    }
    println!("Pb2: {}", sum)
}

fn prime_factors(n: u64) -> HashMap<u64, u64> {
    let mut v = HashMap::new();
    let mut tmp = n;

    let mut a = 2;
    while a <= tmp {
        if tmp % a == 0 {
            tmp /= a;
            *v.entry(a).or_insert(0) += 1;
        } else {
            a += 1
        }
    }
    v
}

fn pb3() {
    let n = 600_851_475_143;
    prime_factors(6);
    let primes = prime_factors(n);
    let m = primes.keys().max().unwrap();
    println!("Pb3: {}", m);
}

fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    let by = s.as_bytes();
    let mut a = 0;
    let mut b = by.len() - 1;

    while a <= b {
        if by[a] != by[b] {
            return false;
        }
        a += 1;
        b -= 1;
    }
    true
}

fn pb4() {
    let mut max_p = 0;
    for i in 100..1000 {
        for j in i..1000 {
            let p = i * j;
            if p > max_p && is_palindrome(p) {
                max_p = p;
            }
        }
    }
    println!("Pb4: {}", max_p)
}

fn pb5() {
    let mut target: HashMap<u64, u64> = HashMap::new();
    for i in 2..21 {
        let f = prime_factors(i);
        for (p, n) in f.iter() {
            let a = *(match target.get(p) {
                Some(vv) => max(vv, n),
                None => n,
            });
            target.insert(*p, a);
        }
    }
    let mut res = 1;
    for (p, n) in target.iter() {
        res *= p.pow(*n as u32);
    }
    println!("Pb5: {}", res);
}

fn pb6() {
    let mut sum_of: u64 = 0;
    let mut square_of: u64 = 0;
    for i in 1..101 {
        sum_of += (i as u64).pow(2);
        square_of += i
    }
    println!("Pb6: {}", square_of.pow(2) - sum_of);
}

fn pb7() {
    let primes = small_primes_nth(10001);
    println!("Pb7: {}", primes.last().unwrap())
}

fn pb8() {
    let number = b"7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";
    const N: usize = 13;
    let mut digits = [0u64; N];
    let mut index = 0;
    let mut max_product = 0;
    for b in number.iter() {
        digits[index] = (b - 48).into();
        index = (index + 1) % N;
        let product = digits.iter().product();
        if product > max_product {
            max_product = product;
        }
    }
    println!("Pb8: {}", max_product);
}

fn pb9() {
    let limit = 500;
    for a in 1..limit {
        for b in a + 1..limit {
            let c2 = (a as u64).pow(2) + (b as u64).pow(2);
            let s = (c2 as f64).sqrt();
            let c = s as u64;
            if a + b + c == 1000 && s.fract() == 0.0 {
                println!("Pb9: {}", a * b * c);
            }
        }
    }
}
fn pb10() {
    let primes = small_primes(2_000_000);
    println!("Pb10: {}", primes.iter().sum::<u64>());
}
fn main() {
    pb1();
    pb2();
    pb3();
    pb4();
    pb5();
    pb6();
    pb7();
    pb8();
    pb9();
    pb10();
}
