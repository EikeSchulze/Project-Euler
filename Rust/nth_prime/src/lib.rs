pub fn nth_prime(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    let mut primes = Vec::new();
    let mut counter = 2;
    while primes.len() < n {
        if !primes.iter().any(|x| counter % x == 0) {
            primes.push(counter);
        }
        counter += 1;
    }
    primes.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_prime() {
        assert_eq!(2, nth_prime(1));
        assert_eq!(3, nth_prime(2));
        assert_eq!(5, nth_prime(3));
        assert_eq!(7, nth_prime(4));
        assert_eq!(11, nth_prime(5));
        assert_eq!(13, nth_prime(6));
    }
}
