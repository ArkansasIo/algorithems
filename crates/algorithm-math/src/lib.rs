pub mod linear;
pub mod statistics;

pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 { let r = a % b; a = b; b = r; }
    a.abs()
}

pub fn is_prime(n: i64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let mut d = 3;
    while d <= n / d { if n % d == 0 { return false; } d += 2; }
    true
}

pub fn sieve(limit: usize) -> Vec<usize> {
    if limit < 2 { return Vec::new(); }
    let mut prime = vec![true; limit + 1];
    prime[0] = false; prime[1] = false;
    let mut p = 2;
    while p <= limit / p {
        if prime[p] { let mut j = p * p; while j <= limit { prime[j] = false; j += p; } }
        p += 1;
    }
    prime.into_iter().enumerate().filter_map(|(i, v)| v.then_some(i)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn prime_test() { assert!(is_prime(97)); assert!(!is_prime(100)); }
    #[test] fn gcd_test() { assert_eq!(gcd(84, 30), 6); }
}
