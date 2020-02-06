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
