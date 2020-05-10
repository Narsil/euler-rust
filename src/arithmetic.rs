pub fn is_prime(n: u64, current_primes: &[u64]) -> bool {
    if n < 2 {
        return false;
    }
    for a in current_primes {
        if n % a == 0 {
            return false;
        }
    }
    true
}
pub fn small_primes_nth(n: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    let mut a = 2;
    while (primes.len() as u64) < n {
        if is_prime(a, &primes) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factors() {
        let primes = small_primes(100);
        assert_eq!(prime_factors(2, &primes), vec![(2, 1)]);
        assert_eq!(prime_factors(6, &primes), vec![(2, 1), (3, 1)]);
        assert_eq!(prime_factors(8, &primes), vec![(2, 3)]);
        assert_eq!(prime_factors(49 * 3, &primes), vec![(3, 1), (7, 2)]);
    }
}
