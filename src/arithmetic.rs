pub fn is_prime(n: u64, current_primes: &[u64]) -> Option<bool> {
    if n < 2 {
        return Some(false);
    }
    for a in current_primes {
        if n < *a {
            return Some(false);
        }
        if n == *a {
            return Some(true);
        }
        if n > *a && n % a == 0 {
            return Some(false);
        }
    }
    let last_prime = current_primes.last().unwrap();
    if n < last_prime * last_prime {
        Some(false)
    } else {
        None
    }
}
pub fn small_primes_nth(n: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    let mut a = 2;
    while (primes.len() as u64) < n {
        if is_prime(a, &primes).is_none() {
            primes.push(a);
        }
        a += 1;
    }
    primes
}
pub fn small_primes(_n: u64) -> Vec<u64> {
    let n = _n as usize;
    let mut primes = Vec::new();
    let mut sieve: Vec<bool> = vec![true; n];
    sieve[0] = false;
    sieve[1] = false;
    let mut prime: usize = 2;
    while prime < n {
        primes.push(prime as u64);
        let mut non_prime = prime * 2;
        while non_prime < n {
            sieve[non_prime] = false;
            non_prime += prime;
        }

        prime += 1;
        while prime < n && !sieve[prime] {
            prime += 1
        }
    }
    primes
}

pub fn prime_factors(n: u64, primes: &[u64]) -> Vec<(u64, u32)> {
    if n <= 1 {
        return vec![];
    }

    let mut number = n;
    let mut prime_factors = vec![];

    for prime in primes {
        let mut power = 0;
        while number % prime == 0 {
            number /= prime;
            power += 1;
        }
        if power > 0 {
            prime_factors.push((*prime, power));
        }
        if number == 1 {
            break;
        }
    }
    prime_factors
}

pub struct DivisorIterator {
    factors: Vec<(u64, u32)>,
    step: Vec<u32>,
}

impl DivisorIterator {
    pub fn new(number: u64, primes: &[u64]) -> DivisorIterator {
        let factors = prime_factors(number, primes);
        let step = vec![0; factors.len()];
        DivisorIterator { factors, step }
    }
}

impl Iterator for DivisorIterator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let divisor: u64 = self
            .factors
            .iter()
            .zip(self.step.iter())
            .map(|((prime, _), power)| prime.pow(*power))
            .product();
        let mut stop = true;
        for (max, power) in self
            .factors
            .iter()
            .map(|(_, max)| max)
            .zip(self.step.iter_mut())
        {
            if *power < *max {
                *power += 1;
                stop = false;
                break;
            }
            // We need to carry
            if stop && *power == *max {
                *power = 0;
            }
        }
        if stop {
            None
        } else {
            Some(divisor)
        }
    }
}

pub fn divisors(number: u64, primes: &[u64]) -> DivisorIterator {
    DivisorIterator::new(number, primes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        let primes = small_primes(100);
        assert_eq!(is_prime(2, &primes), Some(true));
        assert_eq!(is_prime(4, &primes), Some(false));
        assert_eq!(is_prime(256, &primes), Some(false));
        // Exceeded options but 101 < 97**2.
        assert_eq!(is_prime(101, &primes), Some(false));
        // 9479 > 97**2
        assert_eq!(is_prime(9479, &primes), None);
    }

    #[test]
    fn test_factors() {
        let primes = small_primes(100);
        assert_eq!(prime_factors(2, &primes), vec![(2, 1)]);
        assert_eq!(prime_factors(6, &primes), vec![(2, 1), (3, 1)]);
        assert_eq!(prime_factors(8, &primes), vec![(2, 3)]);
        assert_eq!(prime_factors(49 * 3, &primes), vec![(3, 1), (7, 2)]);
    }

    #[test]
    fn test_divisors() {
        let primes = small_primes(100);
        assert_eq!(divisors(2, &primes).collect::<Vec<_>>(), vec![1]);
        assert_eq!(divisors(6, &primes).collect::<Vec<_>>(), vec![1, 2, 3]);
        assert_eq!(
            divisors(220, &primes).collect::<Vec<_>>(),
            vec![1, 2, 4, 5, 10, 20, 11, 22, 44, 55, 110]
        );
    }
}
