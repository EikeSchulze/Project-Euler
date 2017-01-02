pub fn summation_of_primes_below(n: usize) -> u64 {
    if n < 2 {
        return 0;
    }
    let mut sieve = vec![true; n];
    sieve[0] = false;
    sieve[1] = false;

    for i in 2..n {
        if sieve[i] {
            let mut k = 2 * i;
            while k < n {
                sieve[k] = false;
                k += i;
            }
        }
    }

    sieve.into_iter().enumerate().filter(|&(_, is_prime)| is_prime).map(|(i, _)| i as u64).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summation_of_primes_below() {
        assert_eq!(0, summation_of_primes_below(0));
        assert_eq!(0, summation_of_primes_below(1));
        assert_eq!(0, summation_of_primes_below(2));
        assert_eq!(2, summation_of_primes_below(3));
        assert_eq!(5, summation_of_primes_below(4));
        assert_eq!(5, summation_of_primes_below(5));
        assert_eq!(10, summation_of_primes_below(6));
        assert_eq!(10, summation_of_primes_below(7));
        assert_eq!(17, summation_of_primes_below(8));
        assert_eq!(17, summation_of_primes_below(9));
        assert_eq!(17, summation_of_primes_below(10));
    }
}
